use crate::sbi::shutdown;
use core::panic::PanicInfo;

/// #[panic_handler]是一种编译指导属性，用于标记核心库core中的panic!宏要对接的函数
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    if let Some(location) = info.location() {
        println!(
            "Panicked at {}:{} {}",
            location.file(),
            location.line(),
            info.message().unwrap()
        );
    } else {
        println!("[Kernel] Panicked: {}", info.message().unwrap());
    }
    shutdown()
}
