use std::{
    env,
    path::{Path, PathBuf},
};

pub mod bindgen;

pub fn project_root() -> PathBuf {
    Path::new(
        &env::var("CARGO_MANIFEST_DIR").unwrap_or_else(|_| env!("CARGO_MANIFEST_DIR").to_owned()),
    )
    .ancestors()
    .nth(1)
    .unwrap()
    .to_path_buf()
}

pub fn pushd(p: impl AsRef<Path>) -> std::io::Result<Pushd> {
    let cwd = std::env::current_dir()?;
    std::env::set_current_dir(p.as_ref())?;
    Ok(Pushd(cwd))
}
pub struct Pushd(PathBuf);
impl Drop for Pushd {
    fn drop(&mut self) {
        if let Err(e) = std::env::set_current_dir(&self.0) {
            eprintln!("warning: popd failed: {:?}", e);
        }
    }
}
