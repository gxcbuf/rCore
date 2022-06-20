#[panic_handler]
fn panic_handler(panic_info: &core::panic::PanicInfo) -> ! {
    let err = panic_info.message().unwrap();
    if let Some(location) = panic_info.location() {
        println!(
            "[user] Panicked at {}:{}, {}",
            location.file(),
            location.line(),
            err
        );
    } else {
        println!("[user] Panicked: {}", err);
    }
    loop {}
}
