use imgui_sys_bindgen::generate_bindings;
use std::env;

fn main() {
    let cwd = env::current_dir().expect("Failed to read current directory");
    let sys_path = cwd
        .join("..")
        .join("imgui-sys")
        .canonicalize()
        .expect("Failed to find imgui-sys directory");
    let bindings = generate_bindings(&sys_path.join("third-party"), None)
        .expect("Failed to generate bindings");
    let output_path = sys_path.join("src").join("bindings.rs");
    bindings
        .write_to_file(&output_path)
        .expect("Failed to write bindings");

    let wasm_ffi_import_name = option_env!("IMGUI_RS_WASM_IMPORT_NAME")
        .map(|s| s.to_string())
        .or(Some("imgui-sys-v0".to_string()));

    let wasm_bindings = generate_bindings(
        &sys_path.join("third-party"),
        wasm_ffi_import_name,
    )
    .expect("Failed to generate bindings");
    let output_path = sys_path.join("src").join("wasm_bindings.rs");
    wasm_bindings
        .write_to_file(&output_path)
        .expect("Failed to write wasm bindings");
}
