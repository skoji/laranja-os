#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

extern crate rlibc;
use core::panic::PanicInfo;

use laranja_kernel::graphics::{FrameBuffer, ModeInfo};

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBuffer, _mi: *mut ModeInfo) {
    let mut fb = unsafe { *fb };
    unsafe {
        let mut ct = 0;
        while ct < fb.size() {
            fb.write_byte(ct, 255);
            ct += 1;
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
