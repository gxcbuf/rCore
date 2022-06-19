#![no_std]
#![no_main]
#![feature(panic_info_message)]

#[macro_use]
mod console;

mod lang_item;
mod logger;
mod sbi;

use core::arch::global_asm;

use log::{error, info, warn};

global_asm!(include_str!("entry.asm"));

/// #[no_mangle]避免编译器对函数名进行混淆
#[no_mangle]
pub fn rust_main() -> ! {
    extern "C" {
        fn stext();
        fn etext();
        fn srodata();
        fn erodata();
        fn sdata();
        fn edata();
        fn sbss();
        fn ebss();
        fn boot_stack();
        fn boot_stack_top();
    }
    clear_bss();
    println!("Hello, world!");
    logger::init();
    println!("Hello, world!");
    info!("Hello, world!");
    warn!("Hello, world!");
    error!("Hello, world!");

    println!(".text [{:#x}, {:#x})", stext as usize, etext as usize);
    println!(".rodata [{:#x}, {:#x})", srodata as usize, erodata as usize);
    println!(".data [{:#x}, {:#x})", sdata as usize, edata as usize);
    println!(
        "boot_stack [{:#x}, {:#x})",
        boot_stack as usize, boot_stack_top as usize
    );
    println!(".bss [{:#x}, {:#x})", sbss as usize, ebss as usize);

    panic!("Shutdown machine!");
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
