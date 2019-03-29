// Don't use standard library because it won't exist on our OS
#![no_std]
// Don't create an entry point - we will make it ourselves
#![no_main]
// Add the experimental asm! feature
#![feature(asm)]

use core::panic::PanicInfo;


// Panic implementation is required because no standard lib
// Diverging function marked by "never" type, never returns
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // On panic - halt the system
    loop {}
}


/*
    Entry points for Windows - these are the function that will be first called.
    Normally, the compiler generates them for us and calls our main() function,
    but we don't use stdlib and the main wrapping, so we need to make them by ourselves
*/
#[no_mangle]
// This function is for CONSOLE subsystem
pub extern "C" fn mainCRTStartup() -> ! {
    main();
}
#[no_mangle]
pub extern "C" fn main() -> ! {
    loop {}
}