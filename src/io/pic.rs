use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::structures::idt::InterruptStackFrame;

use crate::print;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

// A queue of functions to call when the timer interrupt fires
#[derive(Debug, Clone, Copy)]
pub struct TimerInterruptStack {
    pub stack: [Option<fn()>; 256],
    pub index: usize,
}

impl TimerInterruptStack {
    pub fn push(&mut self, func: fn()) {
        self.stack[self.index] = Some(func);
        self.index += 1;
    }

    pub fn pop(&mut self) -> Option<fn()> {
        let func = self.stack[self.index];
        match func {
            Some(func) => {
                self.index -= 1;
                Some(func)
            },
            None => None,
            
        }
    }
}

lazy_static! {
    pub static ref TIMER_INTERRUPT_STACK: spin::Mutex<TimerInterruptStack> = spin::Mutex::new(TimerInterruptStack {
        stack: [None; 256],
        index: 0,
    });
}

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // Pop the function off the stack and call it
    let func = TIMER_INTERRUPT_STACK.lock().pop();
    match func {
        Some(func) => { 
            print!("-");
            func();
        },
        None => (),
    }


    unsafe {
        // Tell the PIC we are done
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}