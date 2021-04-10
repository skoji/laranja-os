#![feature(abi_efiapi)]
#![no_std]
#![no_main]

#[macro_use]
extern crate alloc;
extern crate rlibc;
use alloc::vec::Vec;
use core::fmt::Write;
use uefi::{prelude::*, proto::media::fs::SimpleFileSystem};
use uefi::{
    proto::media::file::{File, FileAttribute, FileInfo, FileMode, FileType::Regular},
    table::boot::{AllocateType, MemoryType},
};

#[entry]
fn efi_main(_handle: Handle, st: SystemTable<Boot>) -> Status {
    let bt = st.boot_services();
    uefi_services::init(&st).expect_success("Failed to initialize utilities");
    st.stdout()
        .reset(false)
        .expect_success("Failed to reset stdout");
    writeln!(st.stdout(), "Hello from rust").unwrap();

    // open file protocol
    let sfs = if let Ok(sfs) = bt.locate_protocol::<SimpleFileSystem>() {
        let sfs = sfs.expect("cant open filesystem protocol");
        unsafe { &mut *sfs.get() }
    } else {
        writeln!(st.stdout(), "no simple filesystem protocol").unwrap();
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
        let base_pointer = page_pointer as *mut u8;
        let page_buf =
            unsafe { core::slice::from_raw_parts_mut(base_pointer, kernel_file_size as usize) };
        kernel_file.read(page_buf).unwrap().unwrap();

        //
        let code: extern "C" fn() = unsafe { core::mem::transmute(page_pointer + 24) };
        kernel_file.close();
    };

    let mut buffer = Vec::with_capacity(128);
    loop {
        let file_info = match root.read_entry(&mut buffer) {
            Ok(completion) => {
                if let Some(info) = completion.unwrap() {
                    info
                } else {
                    // We've reached the end of the directory
                    break;
                }
            }
            Err(error) => {
                // Buffer is not big enough, allocate a bigger one and try again.
                let min_size = error.data().unwrap();
                buffer.resize(min_size, 0);
                continue;
            }
        };
        writeln!(
            st.stdout(),
            "{}",
            format!("Root directory entry: {:?}", file_info)
        )
        .unwrap();
    }
    root.reset_entry_readout().unwrap().unwrap();

    loop {}
}
