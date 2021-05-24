mod bindgen;
mod flags;
mod submodules;

use anyhow::Result;
use flags::XtaskCmd;
use std::path::{Path, PathBuf};

pub use submodules::autofix_submodules;

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        for cause in e.chain().skip(1) {
            eprintln!("Caused By: {}", cause);
        }
        std::process::exit(-1);
    }
}

fn try_main() -> Result<()> {
    let root = project_root();
    let _d = xshell::pushd(&root)?;
    let flags = flags::Xtask::from_env()?;
    if flags.verbose {
        VERBOSE.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    match flags.subcommand {
        XtaskCmd::Help(_) => eprintln!("{}", flags::Xtask::HELP),
        XtaskCmd::Lint(_) => lint_all()?,
        XtaskCmd::Test(_) => test_all()?,
        XtaskCmd::Modfix(_) => submodules::fixup_submodules()?,
        XtaskCmd::Bindgen(cmd) => cmd.run()?,
    }
    Ok(())
}

fn lint_all() -> Result<()> {
    xshell::cmd!("cargo clippy --workspace --all-targets").run()?;
    xshell::cmd!(
        "cargo clippy --manifest-path imgui-winit-support/Cargo.toml --all-features --all-targets"
    )
    .run()?;
    let winits = &[
        "winit-19",
        "winit-20",
        "winit-22",
        "winit-23/default",
        "winit-24/default",
        "winit-25/default",
    ];
    for &winit in winits {
        xshell::cmd!("cargo clippy --manifest-path imgui-winit-support/Cargo.toml --no-default-features --features {winit} --all-targets").run()?;
    }
    xshell::cmd!("cargo fmt --all -- --check").run()?;
    Ok(())
}

fn test_all() -> Result<()> {
    xshell::cmd!("cargo test --workspace --all-targets").run()?;
    xshell::cmd!("cargo test --workspace --doc").run()?;
    let winits = &[
        "winit-19",
        "winit-20",
        "winit-22",
        "winit-23/default",
        "winit-24/default",
        "winit-25/default",
    ];
    for &winit in winits {
        xshell::cmd!("cargo test --manifest-path imgui-winit-support/Cargo.toml --no-default-features --features {winit} --all-targets").run()?;
    }
    xshell::cmd!("cargo test -p imgui --release -- --ignored").run()?;
    Ok(())
}

static VERBOSE: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);
pub fn verbose() -> bool {
    VERBOSE.load(std::sync::atomic::Ordering::Relaxed)
}

pub fn project_root() -> PathBuf {
    Path::new(
        &std::env::var("CARGO_MANIFEST_DIR")
            .unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}
