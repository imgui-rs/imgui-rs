use crate::flags::Bindgen;
use anyhow::{anyhow, bail, Context, Result};
use std::path::{Path, PathBuf};

impl Bindgen {
    pub fn run(self) -> Result<()> {
        let root = crate::project_root();

        let output = self
            .output_path
            .map(PathBuf::from)
            .unwrap_or_else(|| root.join("imgui-sys/src"));

        let wasm_name = self
            .wasm_import_name
            .or_else(|| std::env::var("IMGUI_RS_WASM_IMPORT_NAME").ok())
            .unwrap_or_else(|| "imgui-sys-v0".to_string());

        for variant in ["master", "docking"] {
            for flag in [None, Some("freetype")] {
                let additional = match flag {
                    None => "".to_string(),
                    Some(x) => format!("-{}", x),
                };
                let cimgui_output = root.join(&format!(
                    "imgui-sys/third-party/imgui-{}{}",
                    variant, additional
                ));

                let types = get_types(&cimgui_output.join("structs_and_enums.json"))?;
                let funcs = get_definitions(&cimgui_output.join("definitions.json"))?;
                let header = cimgui_output.join("cimgui.h");

                let output_name = match (variant, flag) {
                    ("master", None) => "bindings.rs".to_string(),
                    ("master", Some(f)) => format!("{}_bindings.rs", f),
                    (var, None) => format!("{}_bindings.rs", var),
                    (var, Some(f)) => format!("{}_{}_bindings.rs", var, f),
                };

                generate_binding_file(&header, &output.join(&output_name), &types, &funcs, None)?;
                generate_binding_file(
                    &header,
                    &output.join(&format!("wasm_{}", &output_name)),
                    &types,
                    &funcs,
                    Some(&wasm_name),
                )?;
            }
        }

        Ok(())
    }
}

fn get_types(structs_and_enums: &Path) -> Result<Vec<String>> {
    let types_txt = std::fs::read_to_string(structs_and_enums)?;
    let types_val = types_txt
        .parse::<smoljson::ValOwn>()
        .map_err(|e| anyhow!("Failed to parse {}: {:?}", structs_and_enums.display(), e))?;
    let mut types: Vec<String> = types_val["enums"]
        .as_object()
        .ok_or_else(|| anyhow!("No `enums` in bindings file"))?
        .keys()
        .map(|k| format!("^{}", k))
        .collect();
    types.extend(
        types_val["structs"]
            .as_object()
            .ok_or_else(|| anyhow!("No `structs` in bindings file"))?
            .keys()
            .map(|k| format!("^{}", k)),
    );
    Ok(types)
}

fn get_definitions(definitions: &Path) -> Result<Vec<String>> {
    fn bad_arg_type(s: &str) -> bool {
        s == "va_list" || s.starts_with("__")
    }
    let defs_txt = std::fs::read_to_string(definitions)?;
    let defs_val = defs_txt
        .parse::<smoljson::ValOwn>()
        .map_err(|e| anyhow!("Failed to parse {}: {:?}", definitions.display(), e))?;
    let definitions = defs_val
        .into_object()
        .ok_or_else(|| anyhow!("bad json data in defs file"))?;
    let mut keep_defs = vec![];
    for (name, def) in definitions {
        let defs = def
            .into_array()
            .ok_or_else(|| anyhow!("def {} not an array", &name))?;
        keep_defs.reserve(defs.len());
        for func in defs {
            let args = func["argsT"].as_array().unwrap();
            if !args
                .iter()
                .any(|a| a["type"].as_str().map_or(false, bad_arg_type))
            {
                let name = func["ov_cimguiname"]
                    .as_str()
                    .ok_or_else(|| anyhow!("ov_cimguiname wasnt string..."))?;
                keep_defs.push(format!("^{}", name));
            }
        }
    }
    Ok(keep_defs)
}

fn generate_binding_file(
    header: &Path,
    output: &Path,
    types: &[String],
    funcs: &[String],
    wasm_import_mod: Option<&str>,
) -> Result<()> {
    let mut cmd = std::process::Command::new("bindgen");
    let a = &[
        "--size_t-is-usize",
        "--no-prepend-enum-name",
        "--no-doc-comments",
        // Layout tests aren't portable (they hardcode type sizes), and for
        // our case they just serve to sanity check rustc's implementation of
        // `#[repr(C)]`. If we bind directly to C++ ever, we should reconsider this.
        "--no-layout-tests",
        "--with-derive-default",
        "--with-derive-partialeq",
        "--with-derive-eq",
        "--with-derive-hash",
        "--impl-debug",
        "--use-core",
    ];
    cmd.args(a);
    cmd.args(["--blocklist-type", "__darwin_size_t"]);
    cmd.args(["--blocklist-type", "__darwin_off_t"]);
    cmd.args(["--blocklist-file", ".*stdio.h"]);
    cmd.args([
        "--raw-line",
        "#![allow(nonstandard_style, private_interfaces, clippy::all)]",
    ]);
    cmd.args(["--raw-line", "enum FILE {}"]);
    cmd.arg("--output").arg(output);
    cmd.args(["--ctypes-prefix", "cty"]);

    if let Some(name) = wasm_import_mod {
        cmd.args(["--wasm-import-module-name", name]);
    }
    for t in types {
        cmd.args(["--allowlist-type", t]);
    }
    for f in funcs {
        cmd.args(["--allowlist-function", f]);
    }
    cmd.arg(header);
    cmd.args(["--", "-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS=1"]);
    eprintln!("Executing bindgen [output = {}]", output.display());
    let status = cmd.status().context("Failed to execute bindgen")?;
    if !status.success() {
        bail!(
            "Failed to execute bindgen: {}, see output for details",
            status
        );
    }
    eprintln!("Success [output = {}]", output.display());

    Ok(())
}
