#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

use core::panic::PanicInfo;
use laranja_kernel::pci::{read_class_code, read_vendor_id, scan_all_bus, ClassCode};
use laranja_kernel::{console::Console, pci::PciDevices};
use laranja_kernel::{debug, error, info, println};
use laranja_kernel::{
    graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor},
    pci::Device,
};

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

fn list_pci_devices() -> PciDevices {
    let pci_devices = scan_all_bus().unwrap();
    debug!("scanned pci devices.");
    for dev in pci_devices.iter() {
        let vendor_id = read_vendor_id(dev.bus, dev.device, dev.function);
        let class_code = read_class_code(dev.bus, dev.device, dev.function);
        debug!(
            "{}.{}.{}:, vend {:04x}, class {}, head {:02x}",
            dev.bus, dev.device, dev.function, vendor_id, class_code, dev.header_type
        );
    }
    pci_devices
}

fn find_xhc(pci_devices: PciDevices) -> Option<Device> {
    let mut xhc_dev = None;
    let xhcclass = ClassCode {
        base: 0x0c,
        sub: 0x03,
        interface: 0x30,
    };
    for dev in pci_devices.iter() {
        if dev.class_code == xhcclass {
            xhc_dev = Some(dev);
            if dev.get_vendor_id() == 0x8086 {
                break;
            }
        }
    }
    xhc_dev
}

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    initialize(fb, mi);
    draw_mouse_cursor();
    welcome_message();
    let pci_devices = list_pci_devices();
    let xhc = find_xhc(pci_devices);
    match xhc {
        Some(xhc) => {
            info!(
                "xHC has been found: {}.{}.{}",
                xhc.bus, xhc.device, xhc.function
            );
        }
        None => {
            info!("no xHC device");
        }
    };

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
