#![no_std]
#![no_main]
#![feature(abi_efiapi)]
#![feature(asm)]
#![feature(lang_items)]

extern crate rlibc;
use core::panic::PanicInfo;

// #[link_section = ".text.entry"] なくてもいけそう
#[no_mangle]
extern "efiapi" fn kernel_main(mut fb_pt: *mut u8, fb_size: usize) {
    unsafe {
        let mut ct = 0;
        while ct < fb_size {
            *fb_pt = 255;
            fb_pt = fb_pt.add(1);
            ct = ct + 1;
        }
        loop {
            asm!("hlt");
        }
    }
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
