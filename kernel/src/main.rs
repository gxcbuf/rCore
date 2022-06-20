#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;

mod batch;
mod lang_item;
mod logger;
mod sbi;
mod sync;
mod syscall;
mod trap;

use core::arch::global_asm;

use log::{error, info, warn};

global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));

/// #[no_mangle]避免编译器对函数名进行混淆
#[no_mangle]
pub fn rust_main() -> ! {
    clear_bss();
    logger::init();
    println!("[kernel] Hello, world!");
    info!("[kernel] Hello, world!");
    warn!("[kernel] Hello, world!");
    error!("[kernel] Hello, world!");

    trap::init();
    batch::init();
    batch::run_next_app();
}

// 初始化bss段
fn clear_bss() {
    // 尝试从其他地方找到全局符号sbss和ebss
    // extern "C" 可以引用一个外部的C函数接口，意味着调用时遵从目标平台的C语言调用规范
    extern "C" {
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| unsafe { (a as *mut u8).write_volatile(0) });
}
