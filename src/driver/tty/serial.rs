use crate::driver::tty::Tty;

/// A TTY attached via a serial port.
///
/// Serial ports don't commonly exist on physical devices anymore,
/// but many emulators support them and can map them to the host's TTY/terminal emulator,
/// which makes them useful for debugging in a VM.
pub struct SerialTty {
    port: u16,
}

/// The port used by COM1 on x86 devices.
#[cfg(target_arch = "x86_64")]
pub const COM1_PORT: u16 = 0x3F8;

impl SerialTty {
    /// Creates a new serial TTY which will use the provided port for output.
    ///
    /// Unsafe because it is up to the caller to make sure
    /// that the port is actually the port for a TTY device.
    pub unsafe fn new(port: u16) -> SerialTty {
        SerialTty {
            port: port,
        }
    }

    #[cfg(target_arch = "x86_64")]
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
        // This TTY doesn't use buffering.
    }
}
