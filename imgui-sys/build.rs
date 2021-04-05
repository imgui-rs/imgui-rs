#![allow(dead_code)]

use std::fs;
use std::io;
use std::path::Path;

const DEFINES: &[(&str, Option<&str>)] = &[
    // Rust `char` is a unicode scalar value, e.g. 32 bits.
    ("IMGUI_USE_WCHAR32", None),
    // Disabled due to linking issues
    ("CIMGUI_NO_EXPORT", None),
    ("IMGUI_DISABLE_WIN32_FUNCTIONS", None),
    ("IMGUI_DISABLE_OSX_FUNCTIONS", None),
];

fn assert_file_exists(path: &str) -> io::Result<()> {
    match fs::metadata(path) {
        Ok(_) => Ok(()),
        Err(ref e) if e.kind() == io::ErrorKind::NotFound => {
            panic!(
                "Can't access {}. Did you forget to fetch git submodules?",
                path
            );
        }
        Err(e) => Err(e),
    }
}

fn main() -> io::Result<()> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    println!(
        "cargo:THIRD_PARTY={}",
        manifest_dir.join("third-party").display()
    );
    for (key, value) in DEFINES.iter() {
        println!("cargo:DEFINE_{}={}", key, value.unwrap_or(""));
    }
    if std::env::var_os("CARGO_FEATURE_WASM").is_none() {
        // Check submodule status. (Anything else should be a compile error in
        // the C code).
        assert_file_exists("third-party/cimgui.cpp")?;
        assert_file_exists("third-party/imgui/imgui.cpp")?;

        let mut build = cc::Build::new();

        build.cpp(true);
        for (key, value) in DEFINES.iter() {
            build.define(key, *value);
        }

        let compiler = build.get_compiler();
        // Avoid the if-supported flag functions for easy cases, as they're
        // kinda costly.
        if compiler.is_like_gnu() || compiler.is_like_clang() {
            build.flag("-fno-exceptions").flag("-fno-rtti");
        }
        // TODO: disable linking C++ stdlib? Not sure if it's allowed.
        build
            .warnings(false)
            .file("include_all_imgui.cpp")
            .compile("libcimgui.a");
    }
    Ok(())
}
