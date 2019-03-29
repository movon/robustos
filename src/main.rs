// Don't use standard library because it won't exist on our OS
#![no_std]

use core::panic::PanicInfo;


// Panic implementation is required because no standard lib
// Diverging function marked by "never" type, never returns
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    // On panic - halt the system
    loop {}
}


fn main() {}
