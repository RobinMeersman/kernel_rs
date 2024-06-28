#![no_std]
#![no_main]

use core::panic::PanicInfo;


#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {
        // intentionally left empty for now
    }
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {
        // intentionally left empty for now
    }
}
