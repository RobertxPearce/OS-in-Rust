#![no_std] // prevent Rust from loading the standard library
#![no_main] // tell compiler we don't want to use the normal entry point chain

use core::panic::PanicInfo;

// static byte array storing each char as its ascii representation
static HELLO: &[u8] = b"Hello World!";

// this function is the entry point, the linker looks for
// a function named '_start' by default
#[no_mangle] // use no_mangle attribute to prevent name mangling
pub extern "C" fn _start() -> ! {
    // set mutable raw pointer to VGA test buffer address (0xb8000 in x86)
    let vga_buffer= 0xb8000 as *mut u8;
    // for loop to iterate over each byte in 'HELLO'
    for (i, &byte) in HELLO.iter().enumerate() {
        // unsafe block around all memory writes
        unsafe {
            // write the byte and color to the VGA buffer
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}

// define panic handler function
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}