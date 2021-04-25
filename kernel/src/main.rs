#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

use core::fmt::Write;
use core::panic::PanicInfo;
use laranja_kernel::console::Console;
use laranja_kernel::graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};
use laranja_kernel::{print, println};
use log::{error, info, warn};

const BG_COLOR: PixelColor = PixelColor(0, 80, 80);
const FG_COLOR: PixelColor = PixelColor(255, 128, 0);

const MOUSE_CURSOR_HEIGHT: usize = 24;

const MOUSE_CURSOR_SHAPE: [&str; MOUSE_CURSOR_HEIGHT] = [
    "@              ",
    "@@             ",
    "@.@            ",
    "@..@           ",
    "@...@          ",
    "@....@         ",
    "@.....@        ",
    "@......@       ",
    "@.......@      ",
    "@........@     ",
    "@.........@    ",
    "@..........@   ",
    "@...........@  ",
    "@............@ ",
    "@......@@@@@@@@",
    "@......@       ",
    "@....@@.@      ",
    "@...@ @.@      ",
    "@..@   @.@     ",
    "@.@    @.@     ",
    "@@      @.@    ",
    "@       @.@    ",
    "         @.@   ",
    "         @@@   ",
];

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
    info!("Resolution {:?}", graphics.resolution());
    warn!("test waring");
    // draw mouse cursor which will never move
    for (dy, l) in MOUSE_CURSOR_SHAPE.iter().enumerate() {
        for (dx, c) in l.chars().enumerate() {
            let x = 200 + dx;
            let y = 100 + dy;
            match c {
                '@' => {
                    graphics.write_pixel(x, y, &PixelColor(0, 0, 0));
                }
                '.' => {
                    graphics.write_pixel(x, y, &PixelColor(255, 255, 255));
                }
                _ => {}
            }
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
fn panic(info: &PanicInfo) -> ! {
    error!("{}", info);
    loop {}
}
