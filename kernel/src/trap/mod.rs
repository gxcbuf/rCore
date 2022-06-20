mod context;

use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

use crate::batch::run_next_app;
use crate::syscall::syscall;
pub use context::TrapContext;

global_asm!(include_str!("trap.S"));

/// Trap处理总体流程：
/// 1. 通过__alltraps将Trap上下文保存在内核栈上
/// 2. 跳转到trap_handler函数完成Trap分法及处理
/// 3. 当trap_handler返回之后，使用__restore从保存在内核栈上的Trap上下文恢复寄存器
/// 4. 通过sret指令回到应用程序执行

pub fn init() {
    extern "C" {
        fn __alltraps();
    }
    unsafe {
        // 引入外部符号__alltraps,并将stvec设置为Direct模式指向它的地址
        stvec::write(__alltraps as usize, TrapMode::Direct);
    }
}

#[no_mangle]
pub fn trap_handler(ctx: &mut TrapContext) -> &mut TrapContext {
    let scause = scause::read();
    let stval = stval::read();
    match scause.cause() {
        Trap::Exception(Exception::UserEnvCall) => {
            ctx.sepc += 4;
            ctx.x[10] = syscall(ctx.x[17], [ctx.x[10], ctx.x[11], ctx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => panic!(
            "[kernel] Unsupported trap {:?}, stval = {:#x}!",
            scause.cause(),
            stval
        ),
    }
    ctx
}
