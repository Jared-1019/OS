use crate::sbi::shutdown;
use core::panic::PanicInfo;
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let err = info.message().as_str().unwrap_or("(no message)");
    if let Some(location) = info.location() {
        println!("[Kernel] Panicked at {}:{} {}", location.file(), location.line(), err);
    } else {
        println!("[Kernel] Panicked: {}", err);
    }
    shutdown()
}
