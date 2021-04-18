#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

use core::fmt::Write;
use core::panic::PanicInfo;
use laranja_kernel::console::Console;
use laranja_kernel::graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    // initialize Graphics and Console
    unsafe { Graphics::initialize_instance(fb, mi) }
    Console::initialize(&PixelColor(0, 255, 0), &PixelColor(32, 32, 32));

    let graphics = Graphics::instance();

    graphics.clear(&PixelColor(32, 32, 32));
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

    let console = Console::instance();
    console.put_string("Hello from console\na");
    console.put_string("Hello from console again\n");
    console.put_string("Hello from console again and again1\n");
    console.put_string("Hello from console again and again2\n");
    console.put_string("Hello from console again and again3\n");
    console.put_string("Hello from console again and again4\n");
    console.put_string("Hello from console again and again5");
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
