#![feature(abi_efiapi)]
#![no_std]
#![no_main]

use core::panic::PanicInfo;

use uefi::prelude::*;

#[entry]
fn efi_main(image: Handle, system_table: SystemTable<Boot>)-> Status{
    loop{}
}

#[panic_handler]
fn panic_info(info: &PanicInfo)-> !{
    loop{}
}