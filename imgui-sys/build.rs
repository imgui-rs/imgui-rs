#![allow(dead_code)]

use git2::{Oid, Repository};
use std::fs;
use std::io;
use std::path::Path;

#[cfg(not(feature = "docking"))]
const CIMGUI_PATH: &str = "third-party/main/";

#[cfg(feature = "docking")]
const CIMGUI_PATH: &str = "third-party/docking/";

const CPP_FILES: [&str; 4] = [
    "imgui.cpp",
    "imgui_demo.cpp",
    "imgui_draw.cpp",
    "imgui_widgets.cpp",
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

fn get_repository(manifest_dir: &Path) -> Result<Repository, Box<dyn std::error::Error>> {
    let contents = std::fs::read_to_string(&manifest_dir.join("third-party/IMGUI_VERSIONS"))?;
    let versions = contents.split(";").collect::<Vec<_>>();

    #[cfg(not(feature = "docking"))]
    let desired_version = versions[0];

    #[cfg(feature = "docking")]
    let desired_version = versions[1];

    let out_directory = Path::new(&std::env::var("OUT_DIR").expect("no out dir")).join("imgui");

    let repo = match Repository::clone("https://github.com/ocornut/imgui.git", &out_directory) {
        Ok(repo) => repo,
        Err(_) => {
            let repo = Repository::open(&out_directory)?;
            {
                let mut remote = repo.find_remote("origin")?;
                remote.fetch(&["main", "docking"], None, None)?;
            }
            repo
        }
    };

    repo.set_head_detached(Oid::from_str(&desired_version)?)?;
    repo.checkout_head(Some(git2::build::CheckoutBuilder::new().force()))?;
    Ok(repo)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let manifest_dir = Path::new(env!("CARGO_MANIFEST_DIR"));
    let repo = get_repository(&manifest_dir)?;
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
            let path = repo.workdir().unwrap().join(path);
            assert_file_exists(path.to_str().expect("invalid path"))?;
            build.file(path);
        }
        let cimgui_out_path = repo.workdir().unwrap().parent().unwrap();
        for file in &["cimgui.h", "cimgui.cpp"] {
            let in_path = Path::new(CIMGUI_PATH).join(file);
            let out_path = cimgui_out_path.join(file);
            std::fs::copy(in_path, &out_path)?;
        }
        build.file(cimgui_out_path.join("cimgui.cpp"));
        build.compile("libcimgui.a");
    }
    Ok(())
}
