pub fn halt() -> ! {
    use x86_64::instructions::{interrupts, hlt};
    interrupts::disable();
    loop { hlt(); }
}
