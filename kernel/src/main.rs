#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

use core::panic::PanicInfo;
use laranja_kernel::pci::{read_bar, read_class_code, read_vendor_id, scan_all_bus, ClassCode};
use laranja_kernel::usb;
use laranja_kernel::{console::Console, pci::PciDevices};
use laranja_kernel::{debug, error, info, print, trace};
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
    print!(
        r"
    __                             _       ____  _____
   / /   ____ __________ _____    (_)___ _/ __ \/ ___/
  / /   / __ `/ ___/ __ `/ __ \  / / __ `/ / / /\__ \ 
 / /___/ /_/ / /  / /_/ / / / / / / /_/ / /_/ /___/ / 
/_____/\__,_/_/   \__,_/_/ /_/_/ /\__,_/\____//____/  
                            /___/                     

"
    );
    info!("Resolution {:?}", Graphics::instance().resolution());
}

fn list_pci_devices() -> PciDevices {
    let pci_devices = scan_all_bus().unwrap();
    debug!("scanned pci devices.");
    for dev in pci_devices.iter() {
        let vendor_id = read_vendor_id(dev.bus, dev.device, dev.function);
        let class_code = read_class_code(dev.bus, dev.device, dev.function);
        trace!(
            "{}.{}.{}:, vend {:04x}, class {}, head {:02x}",
            dev.bus,
            dev.device,
            dev.function,
            vendor_id,
            class_code,
            dev.header_type
        );
    }
    pci_devices
}

fn find_xhc(pci_devices: &PciDevices) -> Option<Device> {
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

fn switch_echi_to_xhci(_xhc_dev: &Device, pci_devices: &PciDevices) {
    let ehciclass = ClassCode {
        base: 0x0c,
        sub: 0x03,
        interface: 0x20,
    };
    let ehci = pci_devices
        .iter()
        .find(|&dev| dev.class_code == ehciclass && dev.get_vendor_id() == 0x8086);
    if ehci.is_none() {
        info!("no ehci");
    } else {
        panic!("ehci found, but do nothing for the present");
    }
}

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBuffer, mi: *mut ModeInfo) {
    initialize(fb, mi);
    welcome_message();
    let pci_devices = list_pci_devices();
    let xhc = find_xhc(&pci_devices);
    let xhc = match xhc {
        Some(xhc) => {
            info!(
                "xHC has been found: {}.{}.{}",
                xhc.bus, xhc.device, xhc.function
            );
            xhc
        }
        None => {
            panic!("no xHC device");
        }
    };
    switch_echi_to_xhci(&xhc, &pci_devices);
    let xhc_bar = read_bar(&xhc, 0).unwrap();
    debug!("xhc_bar = {:08x}", xhc_bar);
    let xhc_mmio_base = (xhc_bar & !0xf) as usize;
    debug!("xHC mmio_base = {:08x}", xhc_mmio_base);
    unsafe {
        usb::Controller::new(xhc_mmio_base);
    };
    info!("done");
    draw_mouse_cursor();
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
