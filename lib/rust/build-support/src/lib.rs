//! Shared build.rs logic for the workspace's FFI crates.

use std::env;
use std::path::PathBuf;

/// Generate the C++ header `header` for the calling crate's `extern "C"` API.
///
/// The header is written to the build script's OUT_DIR, which
/// scripts/build_rust_lib.py and test/build_rust_lib.py find via cargo's JSON
/// messages and add to the include path. Call from the crate's build.rs.
pub fn generate_header(header: &str) {
    let crate_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let config_path = crate_dir.join("../cbindgen.toml");
    let config = cbindgen::Config::from_file(&config_path).unwrap();

    cbindgen::Builder::new()
        .with_crate(&crate_dir)
        .with_config(config)
        .generate()
        .expect("unable to generate C++ bindings")
        .write_to_file(out_dir.join(header));

    println!("cargo:rerun-if-changed={}", config_path.display());
    println!("cargo:rerun-if-changed=src");
}
