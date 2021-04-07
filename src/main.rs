#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// print to the screen
    #[no_mangle]
    pub extern "C" fn _start() -> ! {
        use core::fmt::Write;
        vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
        write!(vga_buffer::WRITER.lock(), ", some numbers: {} {}", 42, 1.337).unwrap();
    
        loop {}
    }
