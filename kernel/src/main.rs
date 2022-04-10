#![no_std]
#![no_main]
#![feature(lang_items)]
#![feature(custom_test_frameworks)]
#![test_runner(tests::test_runner)]
#![reexport_test_harness_main = "test_main"]

mod ascii_font;
pub mod bitwise_macro;
pub mod console;
pub mod graphics;
pub mod log;
pub mod pci;
pub mod usb;
pub mod volatile;

use log::*;

use console::Console;
use core::arch::asm;
use core::convert::TryFrom;
use core::convert::TryInto;
use core::fmt::Display;
use core::panic::PanicInfo;
use graphics::{FrameBuffer, Graphics, ModeInfo, PixelColor};
use pci::PciDevices;
use pci::{read_bar, read_class_code, read_vendor_id, scan_all_bus, ClassCode, Device};

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

#[repr(C, align(16))]
struct KernelStack<const N: usize> {
    pub stack: [u8; N],
}
const MAIN_STACK_SIZE: usize = 1024 * 1024;

static mut KERNEL_MAIN_STACK: KernelStack<MAIN_STACK_SIZE> = KernelStack {
    stack: [0; MAIN_STACK_SIZE],
};

#[derive(Debug, Copy, Clone)]
pub enum MemoryTypeName {
    Reserved = 0,
    LoaderCode = 1,
    LoaderData = 2,
    BootServicesCode = 3,
    BootServicesData = 4,
    RuntimeServicesCode = 5,
    RuntimeServicesData = 6,
    Conventional = 7,
    Unusable = 8,
    AcpiReclaim = 9,
    AcpiNonVolatile = 10,
    Mmio = 11,
    MmioPortSpace = 12,
    PalCode = 13,
    PersistentMemory = 14,
}

impl TryFrom<u32> for MemoryTypeName {
    type Error = u32;

    fn try_from(v: u32) -> Result<Self, Self::Error> {
        match v {
            v if v == MemoryTypeName::Reserved as u32 => Ok(MemoryTypeName::Reserved),
            v if v == MemoryTypeName::LoaderCode as u32 => Ok(MemoryTypeName::LoaderCode),
            v if v == MemoryTypeName::LoaderData as u32 => Ok(MemoryTypeName::LoaderData),
            v if v == MemoryTypeName::BootServicesCode as u32 => {
                Ok(MemoryTypeName::BootServicesCode)
            }
            v if v == MemoryTypeName::BootServicesData as u32 => {
                Ok(MemoryTypeName::BootServicesData)
            }
            v if v == MemoryTypeName::RuntimeServicesCode as u32 => {
                Ok(MemoryTypeName::RuntimeServicesCode)
            }
            v if v == MemoryTypeName::RuntimeServicesData as u32 => {
                Ok(MemoryTypeName::RuntimeServicesData)
            }
            v if v == MemoryTypeName::Conventional as u32 => Ok(MemoryTypeName::Conventional),
            v if v == MemoryTypeName::Unusable as u32 => Ok(MemoryTypeName::Unusable),
            v if v == MemoryTypeName::AcpiReclaim as u32 => Ok(MemoryTypeName::AcpiReclaim),
            v if v == MemoryTypeName::AcpiNonVolatile as u32 => Ok(MemoryTypeName::AcpiNonVolatile),
            v if v == MemoryTypeName::Mmio as u32 => Ok(MemoryTypeName::Mmio),
            v if v == MemoryTypeName::MmioPortSpace as u32 => Ok(MemoryTypeName::MmioPortSpace),
            v if v == MemoryTypeName::PalCode as u32 => Ok(MemoryTypeName::PalCode),
            v if v == MemoryTypeName::PersistentMemory as u32 => {
                Ok(MemoryTypeName::PersistentMemory)
            }
            v => Err(v),
        }
    }
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct MemoryDescriptor {
    pub memory_type: u32,
    pub padding: u32,
    pub phys_start: u64,
    pub virt_start: u64,
    pub page_count: u64,
    pub att: u64,
}

impl Display for MemoryDescriptor {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match MemoryTypeName::try_from(self.memory_type) {
            Ok(t) => write!(f, "memory_type: {:?}, ", t)?,
            Err(v) => write!(f, "memory_type: {}, ", v)?,
        };
        write!(
            f,
            "padding: 0x{:x} phys_start: 0x{:x}, virt_start: 0x{:x}, page_count: {}, att: {:x}",
            self.padding, self.phys_start, self.virt_start, self.page_count, self.att
        )
    }
}

#[no_mangle]
extern "C" fn kernel_main(
    fb: *mut FrameBuffer,
    mi: *mut ModeInfo,
    memmap_ptr: *mut MemoryDescriptor,
    memmap_length: usize,
) {
    let stack_addr =
        unsafe { (&KERNEL_MAIN_STACK.stack as *const u8).add(MAIN_STACK_SIZE) as usize };
    // move stack to rsp;
    unsafe { asm!("mov rsp, {}", in(reg) stack_addr) };
    kernel_main_newstack(fb, mi, memmap_ptr, memmap_length, stack_addr);
    // should not reach here.
    unsafe {
        loop {
            asm!("hlt");
        }
    }
}

#[no_mangle]
extern "C" fn kernel_main_newstack(
    fb: *mut FrameBuffer,
    mi: *mut ModeInfo,
    memmap_ptr: *mut MemoryDescriptor,
    memmap_length: usize,
    stack_addr: usize,
) {
    initialize(fb, mi);
    welcome_message();
    unsafe { trace!("stack top 0x{:?}", (&KERNEL_MAIN_STACK.stack as *const u8)) };
    trace!("stack bottom : {:x}", stack_addr);
    trace!("memmap_length : {}", memmap_length);
    let memmaps: &[MemoryDescriptor] =
        unsafe { core::slice::from_raw_parts(memmap_ptr, memmap_length) };
    trace!("memmap 0; {}", memmaps[0]);
    trace!("memmap 1; {}", memmaps[1]);
    trace!("memmap 2; {}", memmaps[2]);
    trace!("memmap 3; {}", memmaps[3]);
    trace!("memmap 4; {}", memmaps[4]);
    #[cfg(test)]
    test_main();

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

#[cfg(test)]
mod tests {
    use super::*;
    pub trait TestCaseFn {
        fn run_test(&self);
    }
    impl<T: Fn()> TestCaseFn for T {
        fn run_test(&self) {
            print!("{} ... ", core::any::type_name::<T>());
            self();
            println!("Ok");
        }
    }

    pub fn test_runner(tests: &[&dyn TestCaseFn]) {
        println!("Running tests : {}", tests.len());
        for test in tests {
            test.run_test();
        }
        println!("done.");
    }
}
