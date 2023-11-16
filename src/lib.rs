#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]
#![feature(const_mut_refs)]

pub mod vga;
pub mod idt;
pub mod gdt;
pub mod io;
pub mod print;
pub mod programs;

pub fn init() {
    // Global Descriptor Table
    gdt::init();

    // Interrupt Descriptor Table
    idt::init();

    // Initialize the PIC
    unsafe { io::pic::PICS.lock().initialize() };
    x86_64::instructions::interrupts::enable(); 
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}