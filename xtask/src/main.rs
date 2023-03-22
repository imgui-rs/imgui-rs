mod bindgen;
mod flags;

use anyhow::Result;
use flags::*;
use std::path::{Path, PathBuf};

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
    let _d = xshell::pushd(root)?;
    let flags = flags::Xtask::from_env()?;
    if flags.verbose {
        VERBOSE.store(true, std::sync::atomic::Ordering::Relaxed);
    }
    match flags.subcommand {
        XtaskCmd::Help(_) => eprintln!("{}", flags::Xtask::HELP),
        XtaskCmd::Lint(_) => lint_all()?,
        XtaskCmd::Test(_) => test_all()?,
        XtaskCmd::Bindgen(cmd) => cmd.run()?,
    }
    Ok(())
}

fn lint_all() -> Result<()> {
    // Lint with only default, only docking, and only freetype, and everything
    xshell::cmd!("cargo clippy --workspace --all-targets").run()?;
    xshell::cmd!("cargo clippy --workspace --all-targets --features docking").run()?;
    xshell::cmd!("cargo clippy --workspace --all-targets --features freetype").run()?;
    xshell::cmd!("cargo clippy --workspace --all-targets --all-features").run()?;

    // Check formatting
    xshell::cmd!("cargo fmt --all -- --check").run()?;
    Ok(())
}

fn test_all() -> Result<()> {
    // Test with default/docking/freetype features
    xshell::cmd!("cargo test --workspace --all-targets").run()?;
    xshell::cmd!("cargo test --workspace --all-targets --features docking").run()?;
    xshell::cmd!("cargo test --workspace --all-targets --features freetype").run()?;

    // Test doc examples
    xshell::cmd!("cargo test --workspace --doc").run()?;

    // Run heavy tests in release mode
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
