#![no_std]
#![no_main]
#![feature(asm)]
#![feature(lang_items)]

extern crate rlibc;
use core::panic::PanicInfo;

#[link_section = ".text.entry"]
#[no_mangle]
extern "C" fn kernel_main() {
    loop {
        unsafe {
            //            asm!("mov rax, 60");
            //            asm!("mov rdx, 0");
            //            asm!("syscall");
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
