pub mod gdt;
pub mod idt;

/// This macro exists because the x86_64 library uses `llvm_asm!`, which I have disabled.
/// When the library ever uses plain `asm!` or a function, I will use its version instead.
#[cfg(target_arch="x86_64")]
#[macro_export]
macro_rules! software_interrupt {
    ($x:expr) => {
        asm!("int {}", const $x);
    }
}

pub fn breakpoint() {
    unsafe {
        asm!("int3");
    }
}

pub fn halt() -> ! {
    use x86_64::instructions::{interrupts, hlt};
    interrupts::disable();
    loop { hlt(); }
}
