use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

static mut IDT: InterruptDescriptorTable = InterruptDescriptorTable::new();

pub fn load() {
    unsafe {
        IDT.load();
        IDT.breakpoint.set_handler_fn(breakpoint);
    }
}

extern "x86-interrupt" fn breakpoint(_: &mut InterruptStackFrame) {
    use crate::graphics::tty::Tty;
    use crate::graphics::tty::serial::SerialTty;

    let mut stdout = unsafe { SerialTty::new(0x3F8) };
    stdout.puts("Breakpoint reached!");
}
