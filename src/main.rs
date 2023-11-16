#![no_std]
#![no_main]

use core::panic::PanicInfo;
use rusty_os::{*, programs::shell};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    rusty_os::init();
    
    shell::run();

    // halt the CPU
    hlt_loop();
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);

    hlt_loop();
}
