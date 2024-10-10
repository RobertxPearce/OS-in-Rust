#![no_std] // prevent Rust from loading the standard library
#![no_main] // tell compiler we don't want to use the normal entry point chain

use core::panic::PanicInfo;

// this function is the entry point, the linker looks for
// a function named '_start' by default
#[no_mangle] // use no_mangle attribute to prevent name mangling
pub extern "C" fn _start() -> ! {
    loop {}
}

// define panic handler function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}