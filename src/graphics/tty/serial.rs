use alloc::string::String;
use crate::graphics::tty::Tty;

#[derive(Clone)]
pub struct SerialTty {
    port: u16,
    buffer: String,
}

impl SerialTty {
    pub unsafe fn new(port: u16) -> SerialTty {
        SerialTty {
            port: port,
            buffer: String::new(),
        }
    }

    fn outb(&self, cmd: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.port, in("al") cmd);
        }
    }
}

impl Tty for SerialTty {
    fn putc(&mut self, c: char) {
        self.buffer.push(c);
    }

    fn puts(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    fn clear(&mut self) {
        // VT100 escape code to reset the terminal: `ESC C`.
        self.puts("\u{1B}c");
    }

    fn flush(&mut self) {
        for b in self.buffer.bytes() {
            self.outb(b);
        }
        self.buffer.clear();
    }
}
