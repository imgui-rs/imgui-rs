#![allow(dead_code)]

const DEFINES: &[(&str, Option<&str>)] = &[
    // Rust `char` is a unicode scalar value, e.g. 32 bits.
    ("IMGUI_USE_WCHAR32", None),
    // Disabled due to linking issues
    ("CIMGUI_NO_EXPORT", None),
    ("IMGUI_DISABLE_WIN32_FUNCTIONS", None),
    ("IMGUI_DISABLE_OSX_FUNCTIONS", None),
];

// Output define args for compiler
fn main() -> std::io::Result<()> {
    // Root of imgui-sys
    let manifest_dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));

    #[cfg(feature = "freetype")]
    fn find_freetype() -> Vec<impl AsRef<std::path::Path>> {
        let err_pkg_config;
        let err_vcpkg;
        match pkg_config::Config::new().find("freetype2") {
            Ok(freetype) => return freetype.include_paths,
            Err(err) => err_pkg_config = err,
        }
        match vcpkg::find_package("freetype") {
            Ok(freetype) => return freetype.include_paths,
            Err(err) => err_vcpkg = err,
        }
        panic!(
            "cannot find freetype:\n\
        - pkg-config failed with: {}\n\
        - vcpkg failed with: {}",
            err_pkg_config, err_vcpkg
        );
    }
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
    let wasm_enabled = std::env::var_os("CARGO_FEATURE_WASM").is_some();

    let cimgui_dir = if docking_enabled {
        manifest_dir.join("third-party/imgui-docking")
    } else {
        manifest_dir.join("third-party/imgui-master")
    };

    // For projects like implot-rs we expose the path to our cimgui
    // files, via `DEP_IMGUI_THIRD_PARTY` env-var, so they can build
    // against the same thing
    println!("cargo:THIRD_PARTY={}", cimgui_dir.display());

    // If we aren't building WASM output, bunch of extra stuff to do
    if !wasm_enabled {
        // C++ compiler
        let mut build = cc::Build::new();
        build.cpp(true);

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
            println!("cargo:DEFINE_IMGUI_ENABLE_FREETYPE=");

            // imgui_freetype.cpp needs access to `#include "imgui.h"`.
            // So we include something like '[...]/third-party/imgui-master/imgui/'
            build.include(cimgui_dir.join("imgui"));
        }

        // Which "all imgui" file to use
        let imgui_cpp = if docking_enabled {
            "include_imgui_docking.cpp"
        } else {
            "include_imgui_master.cpp"
        };

        // Set up compiler
        let compiler = build.get_compiler();

        // Avoid the if-supported flag functions for easy cases, as they're
        // kinda costly.
        if compiler.is_like_gnu() || compiler.is_like_clang() {
            build.flag("-fno-exceptions").flag("-fno-rtti");
        }

        // Build imgui lib, suppressing warnings.
        // TODO: disable linking C++ stdlib? Not sure if it's allowed.
        build.warnings(false).file(imgui_cpp).compile("libcimgui.a");
    }
    Ok(())
}
