// Build with -Zbuild-std=core when building for esp32
#![cfg_attr(not(test), no_std)]

#[unsafe(no_mangle)]
pub extern "C" fn rust_lib_add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    unsafe extern "C" {
        fn abort() -> !;
    }
    unsafe { abort() }
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
