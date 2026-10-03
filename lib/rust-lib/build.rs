use std::env;
use std::path::PathBuf;

fn main() {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let config = cbindgen::Config::from_file(crate_dir.join("cbindgen.toml")).unwrap();

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .expect("unable to generate C++ bindings")
        .write_to_file(crate_dir.join("include/rust_lib.h"));

    println!("cargo:rerun-if-changed=cbindgen.toml");
    println!("cargo:rerun-if-changed=src");
    println!("cargo:rerun-if-changed=include/rust_lib.h");
}
