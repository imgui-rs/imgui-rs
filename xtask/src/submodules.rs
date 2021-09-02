use anyhow::Result;
use std::{
    path::{Path, PathBuf},
    sync::atomic::{AtomicBool, Ordering::Relaxed},
};

/// "Automagic" submodule wrangling (sync, clean, fix, etc)
///
/// Ported from rustc's `x.py` code, which runs something like this
/// automatically (even for `./x.py --help`)
///
/// (We only have one submodule, so the handling of multiple submodules is
/// pointless for us for now. However, I snarfed this from the xtask in my game
/// engine, which has several submodules, and we might have more later).
///
/// In theory this can loose local changes made within the submodule. This is
/// unlikely (I asked if this had caused issues within rustc and nobody who
/// replied had experienced it), but should be avoidable if you
///
/// 1. don't modify `.gitignore`d files in submodules unless those files don't
///    need to survive updates.
///
/// 2. don't add modifications to submodules when they're unsynchronized (e.g.
///    doing a `git pull` that needs to update the submodule, and making local
///    modifications to it before syncing it) I don't see a reason for we'd do
///    the first, and the 2nd seems particularly unlikely
///
/// The first won't be an issue for us, and shouldn't be an issue for other
/// projects unless they're doing fairly weird stuff. The second is hard to
/// imagine for us, since most updates to dear imgui are backwards incompatible,
/// so you'll probably notice you need to update them.
pub fn fixup_submodules() -> Result<()> {
    // execute from workspace root
    let _guard = xshell::pushd(&crate::project_root())?;
    let mods = all_submodules()?;
    for path in mods {
        fix_submodule(path.as_ref())?;
    }
    Ok(())
}

/// Same as `fixup_submodules` unless it's explicitly disabled by
/// setting `IMGUI_XTASK_NO_SUBMODULE_FIXUP` in the environment.
///
/// I don't know why you'd need this, but it seems safer to have than not. That
/// said, rustc (where we took our logic from) is way bigger, has a lot more dev
/// going on in submodules, and has no ability to disable the behavior.
pub fn autofix_submodules() {
    if option_env!("IMGUI_XTASK_NO_SUBMODULE_FIXUP").is_none() {
        if let Err(e) = fixup_submodules() {
            eprintln!("Warning: failed to sync submodules: {:?}", e);
        }
    }
}

fn fix_submodule(rel_path: &Path) -> Result<()> {
    let checked_out_hash = {
        // would like to use `cmd` but need
        // https://github.com/matklad/xshell/pull/19
        let out = std::process::Command::new("git")
            .args(&["rev-parse", "HEAD"])
            .current_dir(rel_path)
            .output()?;
        if !out.status.success() {
            anyhow::bail!(
                "`git rev-parse HEAD` (from {}) failed with {:?}",
                rel_path.display(),
                out.status
            );
        }
        String::from_utf8_lossy(&out.stdout).trim().to_string()
    };

    let recorded_hash = {
        let out = std::process::Command::new("git")
            .args(&["ls-tree", "HEAD"])
            .arg(rel_path)
            .output()?;
        if !out.status.success() {
            anyhow::bail!(
                "`git ls-tree HEAD {}` failed with {:?}",
                rel_path.display(),
                out.status,
            );
        }
        let stdout = String::from_utf8_lossy(&out.stdout);
        if stdout.trim().lines().count() != 1 {
            anyhow::bail!("Weird output from git ls-tree: {:?}", stdout)
        }
        // stdout is in the format `mode kind hash\tfilename` and we want `hash`
        stdout.trim().split_whitespace().nth(2).unwrap().to_owned()
    };

    // if the hashes are the same they're the same, we're good
    if checked_out_hash == recorded_hash {
        if crate::verbose() {
            eprintln!(
                "Nothing to be done for {} ({:?} == {:?})",
                rel_path.display(),
                checked_out_hash,
                recorded_hash
            );
        }
        return Ok(());
    }

    // otherwise, update the submodule
    eprintln!("Updating submodule {}", rel_path.display());
    // force it to sync
    xshell::cmd!("git submodule -q sync {rel_path}")
        .echo_cmd(crate::verbose())
        .run()?;

    // NB: rustc supports older version of `git`, and so retries
    // without the --progress arg if running with it fails.
    git_submodule_update_init_recursive()?;
    {
        // enter the submodule and update, reset, and clean to
        // force it to be fully in-sync. If you have unsaved changes,
        // this will lose them (but they can be recovered from the reflog)
        let _d = xshell::pushd(rel_path)?;
        git_submodule_update_init_recursive()?;

        xshell::cmd!("git reset -q --hard")
            .echo_cmd(crate::verbose())
            .run()?;
        xshell::cmd!("git clean -qdfx")
            .echo_cmd(crate::verbose())
            .run()?;
    }
    Ok(())
}

fn all_submodules() -> Result<Vec<PathBuf>> {
    // use `--null` to get \0-separated output, which lets us handle weird shit
    // like paths with `\n` in them.
    let mut out = xshell::cmd!("git config --file .gitmodules --path --null --get-regexp path")
        .echo_cmd(false)
        .output()?;
    // trim the end so that split works.
    while out.stdout.ends_with(b"\0") {
        out.stdout.pop();
    }
    let mut pb = vec![];
    for kv in out.stdout.split(|n| *n == 0).filter(|v| !v.is_empty()) {
        if let Ok(v) = std::str::from_utf8(kv) {
            // `v` is formatted like "{name}\n{path}", so discard everything up
            // to (and including) the newline.
            if let Some(path) = v.find('\n').map(|p| &v[(p + 1)..]) {
                pb.push(path.into());
            } else {
                eprintln!(
                    "warning: invalid format for gitmodule entry: {:?}",
                    String::from_utf8_lossy(kv),
                );
            }
        } else {
            eprintln!(
                "warning: ignoring invalid utf8 in: {:?}",
                String::from_utf8_lossy(kv),
            );
        }
    }
    Ok(pb)
}

/// Note: This is to support older versions of git that don't have `--progress`.
/// I have no idea how old they are, or i'd make a decision on whether to support
/// it. Basically, we run `git submodule update --init --recursive --progress`,
/// and if it fails we omit the `--progress` flag from then on, and immediately
/// retry.
///
/// If we don't care about those git versions, this could just be
/// `xshell::cmd!("git submodule update --init --recursive --progress")`
fn git_submodule_update_init_recursive() -> Result<()> {
    return if NO_PROGRESS_FLAG.load(Relaxed) {
        do_git_smu(false)
    } else if do_git_smu(true).is_err() {
        NO_PROGRESS_FLAG.store(true, Relaxed);
        if crate::verbose() {
            eprintln!("  retrying without `--progress` flag (old git?)");
        }
        do_git_smu(false)
    } else {
        Ok(())
    };

    static NO_PROGRESS_FLAG: AtomicBool = AtomicBool::new(false);

    fn do_git_smu(with_progress_flag: bool) -> Result<()> {
        let flag = if with_progress_flag {
            Some("--progress")
        } else {
            None
        };
        xshell::cmd!("git submodule update --init --recursive {flag...}")
            .echo_cmd(crate::verbose())
            .run()?;
        Ok(())
    }
}
