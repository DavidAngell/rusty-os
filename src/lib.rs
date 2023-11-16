#![no_std]
#![cfg_attr(test, no_main)]
#![feature(abi_x86_interrupt)]

pub mod vga_buffer;
pub mod idt;
pub mod gdt;

pub fn init() {
    gdt::init();
    idt::init();
}