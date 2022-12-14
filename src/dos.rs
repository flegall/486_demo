#[macro_use]
pub mod console;
pub mod allocator;
pub mod error_code;
pub mod file;
pub mod io;
pub mod kbc;
pub mod panic;
pub mod vga;
use core::arch::asm;

pub fn exit(rt: u8) -> ! {
    unsafe {
        asm!("mov ah, 0x4C",
             "int 0x21", in("al") rt);
    }
    loop {}
}
