extern crate imgui_sys_bindgen;

use imgui_sys_bindgen::generate_bindings;
use std::env;

fn main() {
    let cwd = env::current_dir().expect("Failed to read current directory");
    let sys_path = cwd
        .join("..")
        .join("imgui-sys")
        .canonicalize()
        .expect("Failed to find imgui-sys directory");
    let bindings = generate_bindings(&sys_path.join("third-party").join("cimgui"))
        .expect("Failed to generate bindings");
    let output_path = sys_path.join("src").join("bindings.rs");
    bindings
        .write_to_file(&output_path)
        .expect("Failed to write bindings");
    println!("Wrote bindings to {}", output_path.to_string_lossy());
}
