use alloc::string::String;
use alloc::vec::Vec;
use crate::graphics::tty::Tty;
use uefi::proto::console::text::Output;

struct UefiTty<'boot> {
    // It is impossible to get ownership of an Output,
    // so instead we must pass in the entire boot system table.
    output: Output<'boot>,
    buffer: String,
}

impl UefiTty<'_> {
    pub fn new<'boot>(output: Output<'boot>) -> UefiTty<'boot> {
        UefiTty {
            output: output,
            buffer: String::new(),
        }
    }
}

impl Tty for UefiTty<'_> {
    fn putc(&mut self, c: char) {
        self.buffer.push(c);
    }

    fn puts(&mut self, s: &str) {
        self.buffer.push_str(s);
    }

    fn clear(&mut self) {
        // VT100 escape code to reset the terminal: `ESC C`.
        self.output.clear().unwrap().unwrap();
    }

    fn flush(&mut self) {
        let mut codes: Vec<u16> = Vec::new();
        for c in self.buffer.chars() {
            codes.push(c as u16);
        }
        codes.push(0);

        let s = uefi::CStr16::from_u16_with_nul(&codes)
            .unwrap_or_else(|_| panic!("Failed to convert to UCS-2: {}", self.buffer));
        self.output.output_string(s).unwrap().unwrap();

        self.buffer.clear();
    }
}
