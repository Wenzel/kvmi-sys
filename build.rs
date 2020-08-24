extern crate bindgen;
use pkg_config;

use std::env;
use std::path::PathBuf;

fn main() {
    // what library to link with
    println!("cargo:rustc-link-lib=kvmi");

    // find libkvmi
    let libkvmi =
        pkg_config::probe_library("libkvmi").expect("Unable to find libkvmi using pkg-config");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("src/wrapper.h")
        // add libkvmi include paths
        .clang_args(
            libkvmi
                .include_paths
                .iter()
                .map(|inc_path| format!("-I{}", inc_path.display())),
        )
        .derive_copy(true)
        .derive_debug(true)
        .derive_default(true)
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
