#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rusty_os::*;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    
    println!("Hello World{}", "!");
    println!("Hello World{}", "!");
    println!("Hello World{}", "!");
    println!("Hello World{}", "!");
    panic!("Some panic message");

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}
