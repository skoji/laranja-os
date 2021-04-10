#![feature(abi_efiapi)]
#![feature(asm)]
#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;
extern crate rlibc;
use core::fmt::Write;
use uefi::{
    prelude::*,
    proto::{
        console::gop::{FrameBuffer, GraphicsOutput, PixelFormat},
        media::fs::SimpleFileSystem,
    },
    table::boot::MemoryDescriptor,
};
use uefi::{
    proto::media::file::{File, FileAttribute, FileInfo, FileMode, FileType::Regular},
    table::boot::{AllocateType, MemoryType},
};

#[entry]
fn efi_main(handle: Handle, st: SystemTable<Boot>) -> Status {
    let bt = st.boot_services();
    uefi_services::init(&st).expect_success("Failed to initialize utilities");
    let stdout = st.stdout();
    stdout.reset(false).expect_success("Failed to reset stdout");
    writeln!(stdout, "Hello from rust").unwrap();

    // open file protocol
    let sfs = if let Ok(sfs) = bt.locate_protocol::<SimpleFileSystem>() {
        let sfs = sfs.expect("cant open filesystem protocol");
        unsafe { &mut *sfs.get() }
    } else {
        writeln!(stdout, "no simple filesystem protocol").unwrap();
        panic!("no sfs");
    };
    let mut root = sfs.open_volume().unwrap().unwrap();

    let memmap_file = root
        .open("memmap", FileMode::CreateReadWrite, FileAttribute::empty())
        .unwrap()
        .unwrap();
    let memmap_file = memmap_file.into_type().unwrap().unwrap();
    if let Regular(mut memmap_file) = memmap_file {
        let mmap_buf = &mut [0; 4096 * 4];
        let (_, memmap_iter) = bt.memory_map(mmap_buf).unwrap().unwrap();
        memmap_file
            .write("Index, Type, PhysicalStart, NumberOfPages, Attribute\n".as_bytes())
            .unwrap()
            .unwrap();
        for (i, m) in memmap_iter.enumerate() {
            memmap_file
                .write(
                    format!(
                        "{}, {:?}, {}, {}, {:?}\n",
                        i, m.ty, m.phys_start, m.page_count, m.att
                    )
                    .as_bytes(),
                )
                .unwrap()
                .unwrap();
        }
        memmap_file.close();
    };

    let kernel_file = root
        .open("rmikan-kernel", FileMode::Read, FileAttribute::READ_ONLY)
        .unwrap()
        .unwrap();
    let kernel_file = kernel_file.into_type().unwrap().unwrap();
    if let Regular(mut kernel_file) = kernel_file {
        const BUF_SIZE: usize = 4000;
        let buf = &mut [0u8; BUF_SIZE];
        let info: &mut FileInfo = kernel_file.get_info(buf).unwrap().unwrap();
        let kernel_file_size = info.file_size();

        const KERNEL_BASE_ADDRESS: usize = 0x100000;
        let page_pointer = bt
            .allocate_pages(
                AllocateType::Address(KERNEL_BASE_ADDRESS),
                MemoryType::LOADER_DATA,
                (kernel_file_size as usize + 0xfff) / 0x1000,
            )
            .unwrap()
            .unwrap();
        let page_buf = unsafe { &mut *(page_pointer as *mut [u8; 4096]) };
        kernel_file.read(page_buf).unwrap().unwrap();

        //
        let entry_pointer_address: *const u64 = (page_pointer + 24) as *const u64;
        let entry_pointer = unsafe { *entry_pointer_address };
        writeln!(stdout, "entry pointer: {:x}", entry_pointer).unwrap();
        let entry_pointer = entry_pointer as *const ();
        let kernel_entry =
            unsafe { core::mem::transmute::<*const (), extern "C" fn() -> ()>(entry_pointer) };
        kernel_file.close();
        let entry_contents = entry_pointer as *const [u8; 16];
        unsafe {
            for x in &*entry_contents {
                writeln!(st.stdout(), "{:x}", x);
            }
        }
        // graphics in bootloader
        if let Ok(gop) = bt.locate_protocol::<GraphicsOutput>() {
            let gop = gop.expect("Warnings encountered while opening GOP");
            let gop = unsafe { &mut *gop.get() };
            writeln!(
                stdout,
                "resolution {:?}",
                gop.current_mode_info().resolution()
            )
            .unwrap();
            writeln!(
                stdout,
                "pixel format {:?}",
                gop.current_mode_info().pixel_format()
            )
            .unwrap();
            let mi = gop.current_mode_info();
            let stride = mi.stride();
            let (width, height) = mi.resolution();
            let mut fb = gop.frame_buffer();
            type PixelWriter = unsafe fn(&mut FrameBuffer, usize, [u8; 3]);
            unsafe fn write_pixel_rgb(fb: &mut FrameBuffer, pixel_base: usize, rgb: [u8; 3]) {
                fb.write_value(pixel_base, rgb);
            }
            unsafe fn write_pixel_bgr(fb: &mut FrameBuffer, pixel_base: usize, rgb: [u8; 3]) {
                fb.write_value(pixel_base, [rgb[2], rgb[1], rgb[0]]);
            }
            let write_pixel: PixelWriter = match mi.pixel_format() {
                PixelFormat::Rgb => write_pixel_rgb,
                PixelFormat::Bgr => write_pixel_bgr,
                _ => {
                    panic!("This pixel format is not supported by the drawing demo");
                }
            };
            for row in 100..200 {
                for column in 100..200 {
                    unsafe {
                        let pixel_index = (row * stride) + column;
                        let pixel_base = 4 * pixel_index;
                        write_pixel(&mut fb, pixel_base, [255, 0, 0]);
                    }
                }
            }
        }
        // exit boot service
        let max_mmap_size = bt.memory_map_size() + 8 * core::mem::size_of::<MemoryDescriptor>();
        let mut mmap_storage = vec![0; max_mmap_size].into_boxed_slice();
        let (_st, _iter) = st
            .exit_boot_services(handle, &mut mmap_storage[..])
            .expect_success("Failed to exit boot services");
        kernel_entry();
        uefi::Status::LOAD_ERROR
    } else {
        panic!("kernel file is not regular file");
    }
}
