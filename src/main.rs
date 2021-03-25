#![feature(abi_efiapi)]
#![no_std]
#![no_main]

extern crate rlibc;

use core::fmt::Write;
use core::panic::PanicInfo;
use uefi::prelude::*;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {
    writeln!(st.stdout(), "Hello from rust").unwrap();
    loop {}
}
