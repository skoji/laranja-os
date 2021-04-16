#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

extern crate rlibc;
use core::panic::PanicInfo;

use laranja_kernel::graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    let fb = unsafe { *fb };
    let mi = unsafe { *mi };
    let mut graphics = Graphics::new(fb, mi);
    let (width, height) = graphics.resolution();

    unsafe {
        for y in 0..(height / 2) {
            for x in 0..(width / 2) {
                graphics.write_pixel(x, y, PixelColor(250, 0, 0));
            }
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
