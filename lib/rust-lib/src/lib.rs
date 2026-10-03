#[unsafe(no_mangle)]
pub extern "C" fn rust_lib_add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = rust_lib_add(2, 2);
        assert_eq!(result, 4);
    }
}
