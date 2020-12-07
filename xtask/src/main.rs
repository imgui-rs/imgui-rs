use anyhow::Result;
use pico_args::Arguments;
use xtask::bindgen::GenBindings;
use xtask::{project_root, pushd};

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        for cause in e.chain().skip(1) {
            eprintln!("Caused By: {}", cause);
        }
        std::process::exit(-1);
    }
}

const HELP: &str = "\
cargo xtask
Run custom build command.

USAGE:
    cargo xtask <SUBCOMMAND>

SUBCOMMANDS:
    bindgen - produce bindings using bindgen \
        must have bindgen installed, may require unix \
        as we pass `bindgen` very many CLI args
TODO:
    run - run or list examples
    lint-all - run clippy as we would in CI
    test-all - run the tests we'd run in CI
";

fn try_main() -> Result<()> {
    let _g = pushd(project_root())?;

    let mut args = Arguments::from_env();
    let subcommand = args.subcommand()?.unwrap_or_default();

    match subcommand.as_str() {
        "bindgen" => {
            // none of these are required.
            let cmd = GenBindings {
                // defaults to <project>/imgui-sys/third-party
                bindings_path: args.opt_value_from_str("--cimgui-dir")?,
                // defaults to <project>/imgui-sys/src
                output_path: args.opt_value_from_str("--output-dir")?,
                // defaults to "imgui-sys-v0", but can be set in
                // env("IMGUI_RS_WASM_IMPORT_NAME")
                wasm_import_name: args.opt_value_from_str("--wasm-import-name")?,
            };
            args.finish()?;
            cmd.run()?;
        }
        _ => {
            eprintln!("{}", HELP);
        }
    }
    Ok(())
}
