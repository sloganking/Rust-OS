#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

// print to the screen
    #[no_mangle]
    pub extern "C" fn _start() {
        println!("Hello World{}", "!");
        panic!("Some panic message");
        println!("Yeet!");

        loop {}
    }
