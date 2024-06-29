#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(kernel_rs::test_runner)]
#![reexport_test_harness_main = "test_main"]
extern crate core;

use core::panic::PanicInfo;
use kernel_rs::println;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    kernel_rs::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("test println");
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}