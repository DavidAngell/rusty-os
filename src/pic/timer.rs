use x86_64::{instructions::interrupts, structures::idt::InterruptStackFrame};

use crate::{println, print, pic::keyboard::LAST_KEYS_PRESSED};

use super::{InterruptIndex, PICS};
// use super::keypress_queue::KeyQueue;

// Static variable to keep track of the number of timer interrupts
static mut TICKS: u32 = 0;

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    interrupts::without_interrupts(|| {
        // let keys_pressed = LAST_KEYS_PRESSED.lock();

        // // Print the last 256 keys pressed
        // for key in keys_pressed.keys.iter() {
        //     match key {
        //         Some(key) => match key {
        //             pc_keyboard::DecodedKey::Unicode(character) => print!("{}", character),
        //             pc_keyboard::DecodedKey::RawKey(key) => print!("{:?}", key),
        //         },
        //         None => {},
        //     }
        // }

        // // Print a newline
        // println!();
    

        // Increment the number of timer interrupts
        // if unsafe { TICKS } == u32::MAX {
        //     unsafe { TICKS = 0 };
        // } else {
        //     unsafe { TICKS += 1 };
        // }
    });

    unsafe {
        // Tell the PIC we are done
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

pub fn sleep(seconds: u32, callback: fn()) {
    let start_ticks = unsafe { TICKS };
    let end_ticks = start_ticks + seconds;

    loop {
        if unsafe { TICKS } >= end_ticks {
            callback();
            break;
        }
    }
}