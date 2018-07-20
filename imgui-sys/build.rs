extern crate bindgen;
extern crate cc;
extern crate cmake;

use std::env;
use std::path::PathBuf;

fn main() {
    use bindgen::callbacks::*;
    use bindgen::*;

    #[derive(Debug)]
    struct Callbacks;
    // Imgui has enum values prefixed with enum name, we remove prefix using callbacks
    impl ParseCallbacks for Callbacks {
        fn enum_variant_name(
            &self,
            _enum_name: Option<&str>,
            _original_variant_name: &str,
            _variant_value: EnumVariantValue,
        ) -> Option<String> {
            if let Some(enum_name) = _enum_name {
                if let Some(pos) = enum_name.find('_') {
                    return Some(_original_variant_name[pos + 1..].to_string());
                }
            }
            None
        }
    }
    let typedefs = [
        "ImGuiCol",
        "ImGuiDataType",
        "ImGuiDir",
        "ImGuiCond",
        "ImGuiKey",
        "ImGuiNavInput",
        "ImGuiMouseCursor",
        "ImGuiStyleVar",
        "ImDrawCornerFlags",
        "ImDrawListFlags",
        "ImFontAtlasFlags",
        "ImGuiBackendFlags",
        "ImGuiColorEditFlags",
        //"ImGuiColumnsFlags",
        "ImGuiConfigFlags",
        "ImGuiComboFlags",
        "ImGuiDragDropFlags",
        "ImGuiFocusedFlags",
        "ImGuiHoveredFlags",
        "ImGuiInputTextFlags",
        "ImGuiSelectableFlags",
        "ImGuiTreeNodeFlags",
        "ImGuiWindowFlags",
    ];
    // Rename enums that have trailing underscore
    let renames = format!(
        "pub use self::{{ {} }};",
        typedefs
            .iter()
            .map(|t| format!("{}_ as {},", t, t))
            .fold(String::new(), |acc, l| acc + &l)
    );

    // Generate some utility methods, so bindgen generated enums have the same interface
    // as previous handwritten ones
    let impls = typedefs
        .iter()
        .map(|t| {
            format!(
                r#"
impl {typ} {{
    pub fn set(&mut self, flag : {typ}, active : bool) {{
        if active {{
            *self |= flag;
        }} else {{
            *self &= {typ}(!flag.0);
        }}
    }}
}}
       "#,
                typ = t
            )
        })
        .fold(String::new(), |acc, l| acc + &l);

    let mut builder = builder()
        .clang_arg("-Ithird-party/imgui")
        .header("third-party/imgui.hpp")
        .raw_line(renames)
        .raw_line(impls)
        .bitfield_enum(".*");

    // Don't generate enum typedefs
    for t in typedefs.iter() {
        builder = builder.blacklist_type(format!(r#"{}\b"#, t));
    }

    if cfg!(feature = "freetype") {
        builder = builder.header("third-party/freetype.hpp")
    }

    let bindings = builder
        .blacklist_type("max_align_t")
        .derive_partialeq(true)
        .derive_debug(true)
        .disable_name_namespacing()
        .parse_callbacks(Box::new(Callbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());

    bindings
        .write_to_file("debug_ffi.rs")
        .expect("Couldn't write Debug bindings!");

    bindings
        .write_to_file(out_path.join("imgui_ffi.rs"))
        .expect("Couldn't write bindings!");

    let mut build = cc::Build::new();

    build
        .include("third-party/imgui")
        .cpp(true)
        .file("third-party/imgui/imgui.cpp")
        .file("third-party/imgui/imgui_demo.cpp")
        .file("third-party/imgui/imgui_draw.cpp");

    if cfg!(feature = "freetype") {
        build.include("third-party/freetype2/include");
        build.file("third-party/imgui/misc/freetype/imgui_freetype.cpp");
    }

    build.compile("libimgui.a");

    if cfg!(feature = "freetype") {
        let dst = cmake::Config::new("third-party/freetype2")
            .define("WITH_ZLIB", "OFF")
            .define("WITH_HarfBuzz", "OFF")
            .define("WITH_BZip2", "OFF")
            .define("WITH_PNG", "OFF")
            .build();

        println!("cargo:rustc-link-search=native={}/lib", dst.display());
        println!("cargo:rustc-link-search=native={}", dst.display());
        println!("cargo:rustc-link-lib=static=freetyped");
    }
}
