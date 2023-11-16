#![no_std]
#![no_main]

use core::panic::PanicInfo;

use rusty_os::*;

fn stack_overflow() {
    stack_overflow(); // for each recursion, the return address is pushed
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    rusty_os::init();

    stack_overflow();

    // stack_overflow();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    loop {}
}
