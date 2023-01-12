#![no_std]

#[cfg(not(test))] //workaround
use core::panic::PanicInfo;

#[cfg(not(test))] //workaround
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

use libc::printf;

#[no_mangle]
pub extern "C" fn rust_entry() -> i32 {
    unsafe {
        printf("Hello World, from Rust!!!\n\0".as_ptr());
    }

    0
}
