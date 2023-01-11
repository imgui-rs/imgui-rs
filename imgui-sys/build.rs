#![allow(dead_code)]

const DEFINES: &[(&str, Option<&str>)] = &[
    // Rust `char` is a unicode scalar value, e.g. 32 bits.
    ("IMGUI_USE_WCHAR32", None),
    // Disabled due to linking issues
    ("CIMGUI_NO_EXPORT", None),
    ("IMGUI_DISABLE_WIN32_FUNCTIONS", None),
    ("IMGUI_DISABLE_OSX_FUNCTIONS", None),
];

#[cfg(feature = "freetype")]
fn find_freetype() -> Vec<impl AsRef<std::path::Path>> {
    #[cfg(not(feature = "use-vcpkg"))]
    match pkg_config::Config::new().find("freetype2") {
        Ok(freetype) => freetype.include_paths,
        Err(err) => panic!("cannot find freetype: {}", err),
    }
    #[cfg(feature = "use-vcpkg")]
    match vcpkg::find_package("freetype") {
        Ok(freetype) => freetype.include_paths,
        Err(err) => panic!("cannot find freetype: {}", err),
    }
}

// Output define args for compiler
fn main() -> std::io::Result<()> {
    // Root of imgui-sys
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    println!(
        "cargo:THIRD_PARTY={}",
        manifest_dir.join("third-party").display()
    );
    for (key, value) in DEFINES.iter() {
        println!("cargo:DEFINE_{}={}", key, value.unwrap_or(""));
    }

    // Feature flags - no extra dependencies, so these are queried as
    // env-vars to avoid recompilation of build.rs
    let docking_enabled = std::env::var_os("CARGO_FEATURE_DOCKING").is_some();
    let freetype_enabled = std::env::var_os("CARGO_FEATURE_FREETYPE").is_some();
    let wasm_enabled = std::env::var_os("CARGO_FEATURE_WASM").is_some();

    let cimgui_dir = manifest_dir.join(match (docking_enabled, freetype_enabled) {
        (false, false) => "third-party/imgui-master",
        (true, false) => "third-party/imgui-docking",
        (false, true) => "third-party/imgui-master-freetype",
        (true, true) => "third-party/imgui-docking-freetype",
    });

    // For projects like implot-rs we expose the path to our cimgui
    // files, via `DEP_IMGUI_THIRD_PARTY` env-var, so they can build
    // against the same thing
    println!("cargo:THIRD_PARTY={}", cimgui_dir.display());

    // If we aren't building WASM output, bunch of extra stuff to do
    if !wasm_enabled {
        // C++ compiler
        let mut build = cc::Build::new();
        build.cpp(true);

        // imgui uses C++11 stuff from v1.87 onwards
        build.flag_if_supported("-std=c++11");

        // Set defines for compiler
        for (key, value) in DEFINES.iter() {
            build.define(key, *value);
        }

        // Freetype font rasterizer feature
        #[cfg(feature = "freetype")]
        {
            for include in find_freetype() {
                build.include(include);
            }
            // Set flag for dear imgui
            build.define("IMGUI_ENABLE_FREETYPE", None);
            build.define("CIMGUI_FREETYPE", None);
            println!("cargo:DEFINE_IMGUI_ENABLE_FREETYPE=");

            // imgui_freetype.cpp needs access to `#include "imgui.h"`.
            // So we include something like '[...]/third-party/imgui-master/imgui/'
            build.include(dbg!(cimgui_dir.join("imgui")));
        }

        // Which "all imgui" file to use
        let imgui_cpp = match (docking_enabled, freetype_enabled) {
            (false, false) => "include_imgui_master.cpp",
            (true, false) => "include_imgui_docking.cpp",
            (false, true) => "include_imgui_master_freetype.cpp",
            (true, true) => "include_imgui_docking_freetype.cpp",
        };

        // Set up compiler
        let compiler = build.get_compiler();

        // Avoid the if-supported flag functions for easy cases, as they're
        // kinda costly.
        if compiler.is_like_gnu() || compiler.is_like_clang() {
            build.flag("-fno-exceptions").flag("-fno-rtti");
        }

        // Build imgui lib, suppressing warnings.
        build.warnings(false).file(imgui_cpp).compile("libcimgui.a");
    }
    Ok(())
}
