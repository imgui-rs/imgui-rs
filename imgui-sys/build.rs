use std::fs;
use std::io;

const CPP_FILES: [&str; 5] = [
    "third-party/cimgui/cimgui.cpp",
    "third-party/cimgui/imgui/imgui.cpp",
    "third-party/cimgui/imgui/imgui_demo.cpp",
    "third-party/cimgui/imgui/imgui_draw.cpp",
    "third-party/cimgui/imgui/imgui_widgets.cpp",
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
    let mut build = cc::Build::new();
    build.cpp(true);
    for path in &CPP_FILES {
        assert_file_exists(path)?;
        build.file(path);
    }
    build.compile("libcimgui.a");
    Ok(())
}
