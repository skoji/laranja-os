#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

use core::fmt::Write;
use core::panic::PanicInfo;
use laranja_kernel::console::Console;
use laranja_kernel::graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};
use laranja_kernel::{print, println};

static BG_COLOR: PixelColor = PixelColor(32, 32, 32);
static FG_COLOR: PixelColor = PixelColor(255, 128, 0);

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    // initialize Graphics and Console
    unsafe { Graphics::initialize_instance(fb, mi) }
    Console::initialize(&FG_COLOR, &BG_COLOR);

    let graphics = Graphics::instance();

    graphics.clear(&BG_COLOR);
    let (width, _) = graphics.resolution();
    let mut x = width / 3;
    let mut y = 100;
    for i in 0x20u8..0x7Fu8 {
        graphics.write_ascii(x, y, i.into(), &PixelColor(0, 255, 0));
        x += 8;
        if x > width / 3 * 2 {
            x = width / 3;
            y += 32;
        }
    }
    x = width / 3;
    y += 32;
    graphics.write_string(x, y, "Hello, Laranja/Mikan OS", &PixelColor(255, 0, 255));

    let mut writer = graphics.text_writer(width / 3, y + 32, &PixelColor(255, 255, 0));
    writeln!(writer, "1 + 2 = {}", 1 + 2).unwrap();
    println!("Hello Laranja OS !");
    println!("Resolution {:?}", graphics.resolution());
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}

#[lang = "eh_personality"]
fn eh_personality() {}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
