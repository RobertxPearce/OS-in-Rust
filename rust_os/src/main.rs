// Prevent Rust from loading the standard library
#![no_std]

fn main() {}

// Define panic handler function
use core::panic::PanicInfo;
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}