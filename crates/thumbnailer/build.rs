use std::{env};
use std::path::PathBuf;

fn main() {
    // binaries available at https://github.com/bblanchon/pdfium-binaries/releases
    let mut path = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join("libs").join("pdfium");

    let mut valid_config = false;

    if cfg!(target_pointer_width = "64") {
        if cfg!(target_os = "windows") {
            valid_config = true;
            path = path.join("win64").join("pdfium.dll.lib");
        } else if cfg!(target_os = "linux") {
            valid_config = true;
            path = path.join("linux64").join("libpdfium.so");
        }
    }

    if !valid_config {
        println!("cargo:warning=\"Could not statically link pdfium : the configuration is not available !\"")
    } else {
        println!("cargo:rustc-link-search=static=pdfium");
        println!("cargo:rustc-link-lib=native=\"{}\"", path.parent().unwrap().display());
        println!("cargo:rustc-env=PDFIUM_STATIC_LIB_PATH=\"{}\"", path.parent().unwrap().display());
    }
}