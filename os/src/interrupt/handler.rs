use super::context::Context;
use riscv::register::stvec;
use riscv::register::scause::{Scause, Exception, Interrupt, Trap};
use super::timer;
global_asm!(include_str!("./interrupt.asm"));

pub fn init() {
    unsafe {
        extern "C" {
            fn __interrupt();
        }
        stvec::write(__interrupt as usize, stvec::TrapMode::Direct);
    }
}

/// 中断的处理⼊⼝
///
/// `interrupt.asm` ⾸先保存寄存器⾄ Context，其作为参数和 scause 以及 stval ⼀并传⼊此函数
/// 具体的中断类型需要根据 scause 来推断，然后分别处理
#[no_mangle]
pub fn handle_interrupt(context: &mut Context, scause: Scause, stval: usize) {
   // panic!("Interrupted: {:?}", scause.cause());
   match scause.cause() {
       Trap::Exception(Exception::Breakpoint) => breakpoint(context),
       Trap::Interrupt(Interrupt::SupervisorTimer) => supervisor_timer(context),
       _ => fault(context, scause, stval),
   }
}

fn breakpoint(context: &mut Context) {
    println!("Breakpoint at ox{:x}", context.sepc);
    context.sepc += 2;
}

fn supervisor_timer(_: &Context) {
    timer::tick();
}

fn fault(context: &mut Context, scause: Scause, stval: usize) {
    panic!(
        "Unresolved interrupt: {:?}\n{:x?}\nstval: {:x}",
        scause.cause(),
        context,
        stval
    );
}
