#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

use core::panic::PanicInfo;
use laranja_kernel::console::Console;
use laranja_kernel::graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};
use laranja_kernel::pci::{read_class_code, read_vendor_id, scan_all_bus};
use laranja_kernel::{error, info, println};

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

fn initialize(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    unsafe { Graphics::initialize_instance(fb, mi) }
    Console::initialize(&FG_COLOR, &BG_COLOR);
    Graphics::instance().clear(&BG_COLOR);
}

fn draw_mouse_cursor() {
    // draw mouse cursor which will never move
    let graphics = Graphics::instance();
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
}
fn welcome_message() {
    println!("Hello Laranja OS !");
    info!("Resolution {:?}", Graphics::instance().resolution());
}

fn list_pci_devices() {
    let pci_devices = scan_all_bus().unwrap();
    println!("pci devices successfully scanned.");
    for dev in pci_devices.iter() {
        let vendor_id = read_vendor_id(dev.bus, dev.device, dev.function);
        let class_code = read_class_code(dev.bus, dev.device, dev.function);
        println!(
            "{}.{}.{}:, vend {:04x}, class {:08x}, head {:02x}",
            dev.bus, dev.device, dev.function, vendor_id, class_code, dev.header_type
        );
    }
}

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    initialize(fb, mi);
    draw_mouse_cursor();
    welcome_message();
    list_pci_devices();
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
