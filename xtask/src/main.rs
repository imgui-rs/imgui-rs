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
    let _d = xshell::pushd(&root)?;
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
    // Lint with only default, only docking, and only freetype
    xshell::cmd!("cargo clippy --workspace --all-targets").run()?;
    xshell::cmd!("cargo clippy --workspace --all-targets --features docking").run()?;
    xshell::cmd!("cargo clippy --workspace --all-targets --features freetype").run()?;

    // Lint winit with all features
    xshell::cmd!(
        "cargo clippy --manifest-path imgui-winit-support/Cargo.toml --all-features --all-targets"
    )
    .run()?;

    // Lint with various winit versions
    let winits = &[
        "winit-19",
        "winit-20",
        "winit-22",
        "winit-23/default",
        "winit-24/default",
        "winit-25/default",
        "winit-26/default",
    ];
    for &winit in winits {
        xshell::cmd!("cargo clippy --manifest-path imgui-winit-support/Cargo.toml --no-default-features --features {winit} --all-targets").run()?;
    }

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

    // Test with various winit versions
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
