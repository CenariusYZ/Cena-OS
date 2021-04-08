
#![no_std]
#![no_main]
#![feature(global_asm)]
#![feature(llvm_asm)]
#![feature(panic_info_message)]
#![feature(alloc_error_handler)]
#[macro_use]
mod console;
mod panic;
mod sbi;
mod interrupt;
mod memory;

extern crate alloc;

global_asm!(include_str!("entry.asm"));


#[no_mangle]
pub extern "C" fn rust_main() -> ! {
    println!("Hello Cena!");
    interrupt::init();
    memory::init();
    println!("{}", *memory::config::KERNEL_END_ADDRESS);
    //
    use alloc::boxed::Box;
    use alloc::vec::Vec;
    let v = Box::new(5);
    assert_eq!(*v, 5);
    core::mem::drop(v);

    let mut vec = Vec::new();
    for i in 0..10000 {
        vec.push(i);
    }
    assert_eq!(vec.len(), 10000);
    for(i, value) in vec.into_iter().enumerate() {
        assert_eq!(value, i);
    }
    println!("heap test passed by LiaoHaoshen");

    panic!()

}


