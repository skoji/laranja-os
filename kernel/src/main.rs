#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

use core::panic::PanicInfo;

use laranja_kernel::graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    let fb = unsafe { *fb };
    let mi = unsafe { *mi };
    let mut graphics = Graphics::new(fb, mi);

    graphics.clear(&PixelColor(64, 64, 64));
    graphics.write_ascii(100, 100, 'A', &PixelColor(255, 255, 255));
    graphics.write_ascii(108, 100, 'A', &PixelColor(255, 255, 255));
    graphics.write_ascii(116, 100, 'A', &PixelColor(255, 255, 255));
    unsafe {
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
