const VGA_BUFFER_PTR: usize = 0xb8000;

use spin::Mutex;
use lazy_static::lazy_static;

pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH: usize = 80;
lazy_static! {
    pub static ref VGA_BUFFER: Mutex<&'static mut VGABuffer> = Mutex::new(unsafe { &mut *(VGA_BUFFER_PTR as *mut VGABuffer) });
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Color {
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColorCode(u8);
impl ColorCode {
    pub fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[repr(transparent)]
pub struct VGABuffer {
    pub chars: [[VGAChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

impl VGABuffer {
    pub fn clear(&mut self) {
        for row in 0..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                self.chars[row][col] = VGAChar {
                    ascii_character: b' ',
                    color_code: ColorCode::new(Color::Black, Color::Black),
                }
            }
        }
    }
    
    pub fn write_byte(&mut self, row: usize, col: usize, byte: u8, color_code: ColorCode) {
        self.chars[row][col] = VGAChar {
            ascii_character: byte,
            color_code,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VGAChar {
    pub ascii_character: u8,
    pub color_code: ColorCode,
}