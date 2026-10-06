//! Links every workspace crate into librust_lib.a. Each crate's extern "C" API
//! is declared in its own cbindgen header (e.g. rust/epub.h).

// rustc only links dependencies that are referenced, so name each one here.
use epub as _;

#[unsafe(no_mangle)]
pub extern "C" fn rust_lib_add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add() {
        let result = rust_lib_add(2, 2);
        assert_eq!(result, 4);
    }
}
