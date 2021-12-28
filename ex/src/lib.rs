#![no_std]

#[panic_handler]
fn p(_: &core::panic::PanicInfo) -> ! {
    loop{}
}
