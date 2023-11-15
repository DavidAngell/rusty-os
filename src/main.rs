#![no_std]
#![no_main]

use core::panic::PanicInfo;
mod vga;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    vga::print!("Hello World!");
    vga::print!("Hello World!", vga::VGAColor::LightRed);

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
