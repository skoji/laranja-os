#![feature(abi_efiapi)]
#![no_std]
#![no_main]

extern crate rlibc;

use core::fmt::Write;
use uefi::prelude::*;

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {
    uefi_services::init(&st).expect_success("Failed to initialize utilities");
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset stdout");
    writeln!(st.stdout(), "Hello from rust").unwrap();
    loop {}
}
