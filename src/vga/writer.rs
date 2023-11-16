use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

use crate::vga::buffer::{ColorCode, Color, BUFFER_HEIGHT, BUFFER_WIDTH, VGAChar};

lazy_static! {
    pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
        column_position: 0,
        color_code: ColorCode::new(Color::Blue, Color::Black),
    });
}

pub mod ascii {
    pub const LOWER: u8 = 0x20;
    pub const UPPER: u8 = 0x7e;
    pub const NEWLINE: u8 = b'\n';
    pub const UNKNOWN: u8 = 0xfe;
    pub const BLANK: u8 = b' ';
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            ascii::NEWLINE => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                
                // Disable interrupts so we can write to the VGA buffer
                interrupts::without_interrupts(|| {
                    let mut vga_buffer = crate::vga::buffer::VGA_BUFFER.lock();

                    // Write the character to the VGA buffer
                    vga_buffer.chars[row][col] = VGAChar {
                        ascii_character: byte,
                        color_code,
                    };
                });

                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                ascii::LOWER..=ascii::UPPER | ascii::NEWLINE => self.write_byte(byte),
                _ => self.write_byte(ascii::UNKNOWN),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                // Disable interrupts so we can write to the VGA buffer
                interrupts::without_interrupts(|| {
                    let mut vga_buffer = crate::vga::buffer::VGA_BUFFER.lock();

                    // Move the character up one row
                    let character = vga_buffer.chars[row][col];
                    vga_buffer.chars[row - 1][col] = character;
                });
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = VGAChar {
            ascii_character: ascii::BLANK,
            color_code: self.color_code,
        };
        for col in 0..BUFFER_WIDTH {
            // Disable interrupts so we can write to the VGA buffer
            interrupts::without_interrupts(|| {
                let mut vga_buffer = crate::vga::buffer::VGA_BUFFER.lock();

                // Blank out the character
                vga_buffer.chars[row][col] = blank;
            });
        }
    }
}

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}