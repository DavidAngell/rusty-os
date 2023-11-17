use lazy_static::lazy_static;
use x86_64::{structures::idt::InterruptStackFrame, instructions::{port::Port, interrupts}};
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;

use crate::{print, pic::{InterruptIndex, PICS}};

// use super::keypress_queue::KeyQueue;

const KEYBOARD_PORT: u16 = 0x60;

// Create a static keyboard handler
lazy_static! {
    static ref KEYBOARD_HANDLER: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = Mutex::new(
        Keyboard::new(layouts::Us104Key, ScancodeSet1, HandleControl::Ignore)
    );
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut keyboard = KEYBOARD_HANDLER.lock();
    let mut port = Port::new(KEYBOARD_PORT);

    let scancode: u8 = unsafe { port.read() };
    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            interrupts::without_interrupts(|| {
                LAST_KEYS_PRESSED.lock().push(key);
                // KEY_QUEUES.lock().update_all(key);
                match key {
                    DecodedKey::Unicode(character) => print!("{}", character),
                    DecodedKey::RawKey(key) => print!("{:?}", key),
                }
            });
        }
    }

    unsafe {
        // Tell the PIC we are done
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

// An array of the last 256 keys pressed
lazy_static! {
    pub static ref LAST_KEYS_PRESSED: Mutex<LastKeysPressed> = Mutex::new(LastKeysPressed {
        keys: [None; LastKeysPressed::len()],
        index: 0,
    });
}

#[derive(Debug, Clone, Copy)]
pub struct LastKeysPressed {
    pub keys: [Option<DecodedKey>; LastKeysPressed::len()],
    pub index: usize,
}

impl LastKeysPressed {
    pub const fn len() -> usize {
        256
    }

    fn shift_down(&mut self) {
        // Shift all the keys down one
        for i in 0..self.index - 1 {
            self.keys[i] = self.keys[i + 1];
        }

        // Decrement the index
        self.index -= 1;
    }

    pub fn push(&mut self, key: DecodedKey) {
        if self.index >= self.keys.len() {
            self.shift_down();
        }

        self.keys[self.index] = Some(key);
        self.index += 1;
    }
}

// // Maintain an array of KeyQueue structs
// lazy_static! {
//     pub static ref KEY_QUEUES: Mutex<KeyQueueArray> = Mutex::new(KeyQueueArray {
//         queues: [None; 256],
//         index: 0,
//     });
// }

// pub struct KeyQueueArray {
//     pub queues: [Option<KeyQueue>; 256],
//     pub index: usize,
// }

// impl KeyQueueArray {
//     pub fn add(&mut self, queue: KeyQueue) {
//         if self.index >= self.queues.len() {
//             panic!("KeyQueueArray is full!");
//         }

//         self.queues[self.index] = Some(queue);
//         self.index += 1;
//     }

//     pub fn get_queue_mut(&mut self, index: usize) -> &mut KeyQueue {
//         self.queues[index].as_mut().unwrap()
//     }

//     pub fn update_all(&mut self, key: DecodedKey) {
//         for i in 0..self.index {
//             let mut queue = self.get_queue_mut(i);
//             if let Some(key) = queue.dequeue() {
//                 queue.queue(key);
//             }
//         }
//     }
// }
