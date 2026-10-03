use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let config = cbindgen::Config::from_file(crate_dir.join("cbindgen.toml")).unwrap();

    // scripts/build_rust_lib.py finds OUT_DIR via cargo's JSON messages and
    // adds it to the firmware include path.
    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .expect("unable to generate C++ bindings")
        .write_to_file(out_dir.join("rust_lib.h"));

    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=src");
}
