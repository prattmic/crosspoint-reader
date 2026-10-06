//! Links every workspace crate into librust_lib.a. Each crate's extern "C" API
//! is declared in its own cbindgen header (e.g. rust/epub.h).

// rustc only links dependencies that are referenced, so name each one here.
use epub as _;
