#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]


// mod serial;
// mod vga_buffer;

use core::panic::PanicInfo;
use os::println;


// ENTRY POINT
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Zie Voot!{}", "!");

    #[cfg(test)]
    test_main();

    loop {}
}


// PANIC HANDLING
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);

    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}
