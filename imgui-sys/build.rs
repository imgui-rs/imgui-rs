#![allow(dead_code)]

use std::fs;
use std::io;
use std::path::Path;

const CPP_FILES: [&str; 5] = [
    "third-party/cimgui.cpp",
    "third-party/imgui/imgui.cpp",
    "third-party/imgui/imgui_demo.cpp",
    "third-party/imgui/imgui_draw.cpp",
    "third-party/imgui/imgui_widgets.cpp",
];

const DEFINES: &[(&str, Option<&str>)] = &[
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
    #[cfg(not(feature = "wasm"))]
    {
        let mut build = cc::Build::new();
        build.cpp(true);
        for (key, value) in DEFINES.iter() {
            build.define(key, *value);
        }

        build.flag_if_supported("-Wno-return-type-c-linkage");
        for path in &CPP_FILES {
            assert_file_exists(path)?;
            build.file(path);
        }
        build.compile("libcimgui.a");
    }
    Ok(())
}
