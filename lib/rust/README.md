# Rust dependencies

## Adding a new crate

- create lib/rust/<name>/ with an rlib
- add `build-support.workspace = true` to its `[build-dependencies]`
- add a build.rs with `build_support::generate_header("rust_<name>.h");`
- add it to the workspace `members` and `[workspace.dependencies]` in
  lib/rust/Cargo.toml
- add it as a dependency of staticlib with a `use <name> as _;` line
- include `rust_<name>.h` from C++.

## Build system integration

Integrated into platformio build via `scripts/build_rust_lib.py`.

Integrated into test cmake build via `test/build_rust_lib.py`.
