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

    graphics.clear(&PixelColor(32, 32, 32));
    let mut x = 200;
    let mut y = 100;
    let (width, _) = graphics.resolution();

    for i in 0x20u8..0x7Fu8 {
        graphics.write_ascii(x, y, i.into(), &PixelColor(0, 255, 0));
        x += 8;
        if x > width / 3 * 2 {
            x = 200;
            y += 32;
        }
    }
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
