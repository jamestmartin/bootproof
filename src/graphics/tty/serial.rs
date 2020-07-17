use crate::graphics::tty::Tty;

#[derive(Clone)]
pub struct SerialTty {
    port: u16,
}

impl SerialTty {
    pub unsafe fn new(port: u16) -> SerialTty {
        SerialTty {
            port: port,
        }
    }

    fn outb(&self, cmd: u8) {
        unsafe {
            asm!("out dx, al", in("dx") self.port, in("al") cmd);
        }
    }

    fn outc(&self, c: char) {
        let len = c.len_utf8();
        let bytes = (c as u32).to_le_bytes();
        for i in 0..len {
            self.outb(bytes[i]);
        }
    }
}

impl Tty for SerialTty {
    fn putc(&mut self, c: char) {
        self.outc(c);
    }

    fn puts(&mut self, s: &str) {
        for c in s.chars() {
            self.putc(c);
        }
    }

    fn clear(&mut self) {
        // VT100 escape code to reset the terminal: `ESC C`.
        self.puts("\u{1B}c");
    }

    fn flush(&mut self) {
        // This TTY doesn't support buffering.
    }
}
