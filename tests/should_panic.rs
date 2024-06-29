#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use kernel_rs::{exit_qemu, QemuExitCode, serial_print, serial_println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[Ok]");
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    loop {}
}