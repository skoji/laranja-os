#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

extern crate rlibc;
use core::panic::PanicInfo;

// #[link_section = ".text.entry"] なくてもいけそう
#[no_mangle]
extern "C" fn kernel_main() {
    unsafe {
        asm!("hlt");
    }
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
