#![no_std]
#![no_main]

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// #[repr(u8)]
// pub enum Color {
//     Black = 0,
//     Blue = 1,
//     Green = 2,
//     Cyan = 3,
//     Red = 4,
//     Magenta = 5,
//     Brown = 6,
//     LightGray = 7,
//     DarkGray = 8,
//     LightBlue = 9,
//     LightGreen = 10,
//     LightCyan = 11,
//     LightRed = 12,
//     Pink = 13,
//     Yellow = 14,
//     White = 15,
// }

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
