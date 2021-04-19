#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

use core::{mem::MaybeUninit, panic::PanicInfo};
use laranja_kernel::console::Console;
use laranja_kernel::graphics::{Graphics, PixelColor};
use laranja_kernel::{print, println};
use uefi::proto::console::gop::GraphicsOutput;

static mut GOP: MaybeUninit<GraphicsOutput> = MaybeUninit::<GraphicsOutput>::uninit();

#[no_mangle]
extern "C" fn kernel_main(gop: *mut GraphicsOutput<'static>) {
    unsafe {
        core::ptr::copy(gop, GOP.as_mut_ptr(), 1);
    }
    // initialize Graphics and Console
    unsafe { Graphics::initialize_instance(&mut *GOP.as_mut_ptr()) };
    Console::initialize(&PixelColor(255, 128, 0), &PixelColor(32, 32, 32));

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

    println!("Hello Laranja OS !");
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
