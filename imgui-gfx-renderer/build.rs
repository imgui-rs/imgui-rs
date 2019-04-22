fn main() {
    #[cfg(windows)]
    hlsl_build::compile_hlsl_shaders();
}

#[cfg(windows)]
mod hlsl_build {
    use std::env;
    use std::ffi::CStr;
    use std::ffi::CString;
    use std::fs;
    use std::path::Path;
    use std::path::PathBuf;
    use std::ptr;
    use std::slice;
    use std::str;

    pub fn compile_hlsl_shaders() {
        let source_path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap())
            .join("src")
            .join("shader")
            .join("sm_40.hlsl");

        println!("cargo:rerun-if-changed={}", source_path.display());

        let src_data = fs::read_to_string(&source_path).unwrap();

        let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

        fs::write(
            out_dir.join("hlsl_vertex_shader_bytecode"),
            compile_shader(&src_data, &source_path, "VertexMain", "vs_4_0").unwrap_or_else(
                |error_message| {
                    eprintln!("{}", error_message);
                    panic!("Vertex shader failed to compile");
                },
            ),
        )
        .unwrap();

        fs::write(
            out_dir.join("hlsl_pixel_shader_bytecode"),
            compile_shader(&src_data, &source_path, "PixelMain", "ps_4_0").unwrap_or_else(
                |error_message| {
                    eprintln!("{}", error_message);
                    panic!("Pixel shader failed to compile");
                },
            ),
        )
        .unwrap();
    }

    fn compile_shader(
        src_data: &str,
        source_path: &Path,
        entry_point: &str,
        target: &str,
    ) -> Result<Vec<u8>, String> {
        use winapi::shared::minwindef::LPCVOID;
        use winapi::um::d3dcommon::ID3DBlob;
        use winapi::um::d3dcompiler;

        unsafe {
            let mut code: *mut ID3DBlob = ptr::null_mut();
            let mut error_msgs: *mut ID3DBlob = ptr::null_mut();

            let hr = d3dcompiler::D3DCompile(
                src_data.as_bytes().as_ptr() as LPCVOID,
                src_data.as_bytes().len(),
                CString::new(source_path.to_string_lossy().to_string())
                    .unwrap()
                    .as_ptr(),
                ptr::null(),
                ptr::null_mut(),
                CString::new(entry_point).unwrap().as_ptr(),
                CString::new(target).unwrap().as_ptr(),
                0,
                0,
                &mut code,
                &mut error_msgs,
            );

            if hr < 0 {
                if !error_msgs.is_null() {
                    let error_msgs = error_msgs.as_ref().unwrap();

                    let error_msgs = str::from_utf8(slice::from_raw_parts(
                        error_msgs.GetBufferPointer() as *const u8,
                        error_msgs.GetBufferSize(),
                    ))
                    .expect("error messages from D3DCompile not valid UTF-8");

                    Err(error_msgs.to_string())
                } else {
                    Err(format!("hresult: {}", hr))
                }
            } else {
                let code = code
                    .as_ref()
                    .expect("null code blob returned from D3DCompile");

                Ok(slice::from_raw_parts(
                    code.GetBufferPointer() as *const u8,
                    code.GetBufferSize(),
                )
                .to_vec())
            }
        }
    }

}
