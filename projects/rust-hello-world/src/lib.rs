#![no_std]

#[cfg(not(test))] //workaround
use core::panic::PanicInfo;

#[cfg(not(test))] //workaround
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn rust_entry() -> i32 {
    0
}
