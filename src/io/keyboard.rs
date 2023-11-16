use lazy_static::lazy_static;
use x86_64::{structures::idt::InterruptStackFrame, instructions::port::Port};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;

use crate::{print, io::pic::{InterruptIndex, PICS}};

const KEYBOARD_PORT: u16 = 0x60;

// Create a static keyboard instance
lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD.lock();
    let mut port = Port::new(KEYBOARD_PORT);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => print!("{}", character),
                DecodedKey::RawKey(key) => print!("{:?}", key),
            }
        }
    }

    unsafe {
        // Tell the PIC we are done
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
