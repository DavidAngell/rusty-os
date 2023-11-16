use x86_64::instructions::interrupts;

use crate::{println, io::pic::TIMER_INTERRUPT_STACK};

pub fn run() {
    println!("Hello from shell.rs");
    let mut stack = TIMER_INTERRUPT_STACK.lock();
    for _ in 0..100 {
        interrupts::without_interrupts(|| {
            stack.push(|| println!("."));
        });
    }

    drop(stack); // drop the lock

    // Do some other stuff
}