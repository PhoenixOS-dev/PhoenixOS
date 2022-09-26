#![no_std]
#![no_main]

use core::panic::PanicInfo;

extern "C" fn kernel_main() -> !{
    loop{}
}

#[panic_handler]
fn panic_info(info: &PanicInfo) -> !{
    loop{}
}