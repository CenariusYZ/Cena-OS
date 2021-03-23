
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]

#[macro_use]
mod console;
mod panic;
mod sbi;

global_asm!(include_str!("entry.asm"));


#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello CenariusYZ!");
    panic!("end of rust_main")
}


