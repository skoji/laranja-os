#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

extern crate rlibc;
use core::panic::PanicInfo;

//  copyied from uefi-rs
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(u32)]
pub enum PixelFormat {
    /// Each pixel is 32-bit long, with 24-bit RGB, and the last byte is reserved.
    Rgb = 0,
    /// Each pixel is 32-bit long, with 24-bit BGR, and the last byte is reserved.
    Bgr,
    /// Custom pixel format, check the associated bitmask.
    Bitmask,
    /// The graphics mode does not support drawing directly to the frame buffer.
    ///
    /// This means you will have to use the `blt` function which will
    /// convert the graphics data to the device's internal pixel format.
    BltOnly,
    // SAFETY: UEFI also defines a PixelFormatMax variant, and states that all
    //         valid enum values are guaranteed to be smaller. Since that is the
    //         case, adding a new enum variant would be a breaking change, so it
    //         is safe to model this C enum as a Rust enum.
}
/// Bitmask used to indicate which bits of a pixel represent a given color.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[repr(C)]
pub struct PixelBitmask {
    /// The bits indicating the red channel.
    pub red: u32,
    /// The bits indicating the green channel.
    pub green: u32,
    /// The bits indicating the blue channel.
    pub blue: u32,
    /// The reserved bits, which are ignored by the video hardware.
    pub reserved: u32,
}

#[derive(Debug, Copy, Clone)]
#[repr(C)]
pub struct ModeInfo {
    pub version: u32,
    pub hor_res: u32,
    pub ver_res: u32,
    pub format: PixelFormat,
    pub mask: PixelBitmask,
    pub stride: u32,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
struct FrameBufferInfo {
    pub fb: *mut u8,
    pub size: usize,
}

#[no_mangle]
extern "C" fn kernel_main(fb: *mut FrameBufferInfo, _mi: *mut ModeInfo) {
    let fb = unsafe { *fb };
    let mut fb_pt = fb.fb;
    let fb_size = fb.size;
    unsafe {
        let mut ct = 0;
        while ct < fb_size {
            *fb_pt = 255;
            fb_pt = fb_pt.add(1);
            ct += 1;
        }
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
