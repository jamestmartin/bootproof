pub mod graphic;

use alloc::boxed::Box;

/// A text-mode display. Basically, an array of characters that you can set in any order.
pub trait TextDisplay {
    fn borrow_frame<'a>(&'a self) -> &'a TextDisplayFrame;
    fn borrow_mut_frame<'a>(&'a mut self) -> &'a mut TextDisplayFrame;
    /// Display all changes made to the frame.
    fn refresh(&mut self);
}

/// A frame of a text display; basically a 2d array of characters which you can set how you please.
/// However, this frame doesn't know anything about how to display itself;
/// that's what the TextDisplay trait is for.
pub struct TextDisplayFrame {
    resolution: (usize, usize),
    buf: Box<[char]>,
}

impl TextDisplayFrame {
    pub fn new(resolution: (usize, usize)) -> TextDisplayFrame {
        use alloc::vec::Vec;

        let (width, height) = resolution;
        let mut buf = Vec::new();
        buf.resize(width * height, '\u{0}');

        TextDisplayFrame {
            resolution: resolution,
            buf: buf.into_boxed_slice()
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

    /// Set all characters in this frame to null.
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
