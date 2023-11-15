#[repr(u8)]
pub enum VGAColor {
    Black = 0,
    Blue = 1,
    Green = 2,
    Cyan = 3,
    Red = 4,
    Magenta = 5,
    Brown = 6,
    LightGray = 7,
    DarkGray = 8,
    LightBlue = 9,
    LightGreen = 10,
    LightCyan = 11,
    LightRed = 12,
    Pink = 13,
    Yellow = 14,
    White = 15,
}

#[macro_export]
macro_rules! print {
    ($string:expr) => {
        let vga_buffer = 0xb8000 as *mut u8;

        for (i, &byte) in $string.as_bytes().iter().enumerate() {
            use crate::vga::VGAColor;
            
            unsafe {
                *vga_buffer.offset(i as isize * 2) = byte;
                *vga_buffer.offset(i as isize * 2 + 1) = VGAColor::White as u8;
            }
        }
    };

    ($string:expr, $color:expr) => {
        let vga_buffer = 0xb8000 as *mut u8;

        for (i, &byte) in $string.as_bytes().iter().enumerate() {
            unsafe {
                *vga_buffer.offset(i as isize * 2) = byte;
                *vga_buffer.offset(i as isize * 2 + 1) = $color as u8;
            }
        }
    };
}

pub use print;