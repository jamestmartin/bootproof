use alloc::vec::Vec;

pub struct TerminalFrame {
    resolution: (usize, usize),
    buf: Vec<char>,
}

impl TerminalFrame {
    pub fn new(resolution: (usize, usize)) -> TerminalFrame {
        let (width, height) = resolution;
        let buf_length = width * height;
        let mut buf = Vec::with_capacity(buf_length);
        for _ in 0..buf_length {
            buf.push('\u{0}');
        }

        TerminalFrame {
            resolution: resolution,
            buf: buf
        }
    }

    pub fn resolution(&self) -> (usize, usize) {
        self.resolution
    }

    pub fn width(&self) -> usize {
        self.resolution.0
    }

    pub fn height(&self) -> usize {
        self.resolution.1
    }

    pub fn clear(&mut self) {
        for i in 0..self.buf.len() {
            self.buf[i] = '\u{0}';
        }
    }

    fn index(&self, x: usize, y: usize) -> usize {
        self.width() * y + x
    }

    pub fn get(&self, x: usize, y: usize) -> char {
        self.buf[self.index(x, y)]
    }

    pub fn set(&mut self, x: usize, y: usize, c: char) {
        let i = self.index(x, y);
        self.buf[i] = c;
    }
}
