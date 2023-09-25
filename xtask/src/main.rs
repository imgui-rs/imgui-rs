use anyhow::Result;
use camino::Utf8Path;
use clap::Parser;

/// This is a build utility for generating bindings for a new imgui file.
/// We use this, rather than a shell or bash file, since contributors know Rust,
/// and we want to allow the use of Rust to maintain this project.
#[derive(Parser)]
struct Cli {
    #[clap(default_value = "./imgui-sys2/third-party")]
    third_party: camino::Utf8PathBuf,

    #[clap(default_value = "./imgui-sys2/src")]
    output_folder: camino::Utf8PathBuf,

    #[clap(short)]
    verbose: bool,
}

fn main() {
    if let Err(e) = try_main() {
        eprintln!("{}", e);
        for cause in e.chain().skip(1) {
            eprintln!("Caused By: {}", cause);
        }
        std::process::exit(1);
    }
}

fn try_main() -> Result<()> {
    let cli = Cli::parse();

    #[allow(clippy::single_element_loop)]
    for variant in ["master" /* "docking" */] {
        // let additional = match flag {
        //     None => "".to_string(),
        //     Some(x) => format!("-{}", x),
        // };
        let dear_bindings = cli.third_party.join(&format!(
            "imgui-{}",
            variant,
            // additional
        ));

        let data: DearBindingsJson = serde_json::from_str(
            &std::fs::read_to_string(dear_bindings.join("cimgui.json"))
                .expect("no dear_bindings.json"),
        )
        .expect("bad bindings?");
        let mut types: Vec<String> = data.structs.into_iter().map(|v| v.name).collect();
        if cli.verbose {
            println!("generating the following structs: \n{:#?}", types);
            println!(
                "generating the following enums: \n{:#?}",
                Vec::from_iter(data.enums.iter().map(|v| &v.name))
            );
        }

        types.extend(data.enums.into_iter().map(|v| v.name));
        types.push("ImGuiKey_".to_string());

        let funcs: Vec<_> = data
            .functions
            .into_iter()
            .filter_map(|func| {
                let valid = !func.arguments.iter().any(|arg| arg.is_varargs);
                valid.then_some(func.name)
            })
            .collect();
        let header = dear_bindings.join("cimgui.h");
        let output_name = "exp_bindings.rs";

        // let output_name = match (variant, flag) {
        //     ("master", None) => "bindings.rs".to_string(),
        //     ("master", Some(f)) => format!("{}_bindings.rs", f),
        //     (var, None) => format!("{}_bindings.rs", var),
        //     (var, Some(f)) => format!("{}_{}_bindings.rs", var, f),
        // };

        generate_binding_file(
            &header,
            &cli.output_folder.join(output_name),
            &types,
            &funcs,
        )?;
    }

    Ok(())
}

#[derive(serde::Deserialize)]
struct DearBindingsJson {
    enums: Vec<StringHolder>,
    structs: Vec<StringHolder>,
    functions: Vec<Function>,
}

/// We don't need the rest that serde can parse, so we just bind a name
#[derive(serde::Deserialize)]
struct StringHolder {
    name: String,
}

#[derive(serde::Deserialize)]
struct Function {
    name: String,
    arguments: Vec<Argument>,
}

#[derive(serde::Deserialize)]
struct Argument {
    is_varargs: bool,
}

fn generate_binding_file(
    header: &Utf8Path,
    output: &Utf8Path,
    types: &[String],
    funcs: &[String],
) -> Result<()> {
    let mut builder = bindgen::Builder::default()
        .prepend_enum_name(false)
        .layout_tests(false)
        .derive_default(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .derive_hash(true)
        .impl_debug(true)
        .use_core()
        .blocklist_type("__darwin_size_t")
        .raw_line("#![allow(nonstandard_style, clippy::all)]")
        .ctypes_prefix("cty")
        .clang_arg("-DIMGUI_USE_WCHAR32=1")
        .header(header.as_str());

    for t in types {
        builder = builder.allowlist_type(t);
    }

    for t in funcs {
        builder = builder.allowlist_function(t);
    }

    eprintln!("Executing bindgen...");
    let bindgen_output = builder.generate();

    match bindgen_output {
        Ok(v) => {
            v.write_to_file(output).unwrap();
            eprintln!("Success [output = {}]", output);
        }
        Err(e) => {
            anyhow::bail!("Failed to execute bindgen: {}, see output for details", e);
        }
    }

    Ok(())
}
