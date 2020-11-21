use bindgen::{Bindings, EnumVariation, RustTarget};
use serde_derive::Deserialize;
use std::collections::HashMap;
use std::error::Error;
use std::fmt;
use std::fs::{read_to_string, File};
use std::io::Read;
use std::path::Path;

#[derive(Debug)]
struct BindgenError;

impl fmt::Display for BindgenError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Failed to generate bindings")
    }
}

impl Error for BindgenError {}

#[derive(Deserialize)]
struct StructsAndEnums {
    enums: HashMap<String, serde_json::Value>,
    structs: HashMap<String, serde_json::Value>,
}

#[derive(Deserialize)]
struct DefinitionArg {
    #[serde(rename = "type")]
    type_: String,
}

#[derive(Deserialize)]
struct Definition {
    #[serde(rename = "argsT")]
    args_t: Vec<DefinitionArg>,
    ov_cimguiname: String,
}

#[derive(Debug, Clone)]
struct Whitelist {
    enums: Vec<String>,
    structs: Vec<String>,
    definitions: Vec<String>,
}

fn parse_whitelist<R: Read>(
    structs_and_enums: R,
    definitions: R,
) -> Result<Whitelist, serde_json::Error> {
    let StructsAndEnums { enums, structs } = serde_json::from_reader(structs_and_enums)?;
    let enums = enums.keys().cloned().collect();
    let structs = structs.keys().cloned().collect();

    let definitions: HashMap<String, Vec<Definition>> = serde_json::from_reader(definitions)?;
    let definitions = definitions
        .into_iter()
        .flat_map(|(_, defs)| defs.into_iter())
        .filter_map(|d| {
            let uses_va_list = d.args_t.iter().any(|a| a.type_ == "va_list");
            if uses_va_list {
                None
            } else {
                Some(d.ov_cimguiname)
            }
        })
        .collect();

    Ok(Whitelist {
        enums,
        structs,
        definitions,
    })
}

pub fn generate_bindings<P: AsRef<Path>>(
    path: &P,
    wasm_import_name: Option<String>,
) -> Result<Bindings, Box<dyn Error>> {
    let path = path.as_ref();
    let structs_and_enums = File::open(path.join("structs_and_enums.json"))?;
    let definitions = File::open(path.join("definitions.json"))?;
    let header = read_to_string(path.join("cimgui.h"))?;

    let whitelist = parse_whitelist(structs_and_enums, definitions)?;
    let mut builder = bindgen::builder()
        .header_contents("cimgui.h", &header)
        .rust_target(RustTarget::Stable_1_40)
        .default_enum_style(EnumVariation::Consts)
        .size_t_is_usize(true)
        .prepend_enum_name(false)
        .generate_comments(false)
        .layout_tests(true)
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        .derive_hash(true)
        .derive_partialeq(true)
        .derive_eq(true)
        .impl_debug(true)
        .rustfmt_bindings(true)
        .clang_arg("-DCIMGUI_DEFINE_ENUMS_AND_STRUCTS=1");

    if let Some(name) = wasm_import_name {
        builder = builder.wasm_import_module_name(name);
    }

    for e in whitelist.structs {
        builder = builder.whitelist_type(format!("^{}", e));
    }
    for e in whitelist.enums {
        builder = builder.whitelist_type(format!("^{}", e));
    }
    for e in whitelist.definitions {
        builder = builder.whitelist_function(format!("^{}", e));
    }
    let bindings = builder.generate().map_err(|_| BindgenError)?;
    Ok(bindings)
}
