use alloc::sync::Arc;
use alloc::vec::Vec;
use crate::graphics::color::{Color, RGB};
use crate::graphics::display::Display;
use crate::graphics::display::gop::GopDisplay;
use crate::graphics::font::psf::PSF;
use crate::graphics::terminal::Terminal;

pub struct DisplayTerminal<'a> {
    dp: GopDisplay<'a>,
    font: Arc<PSF>,
    buf: Vec<char>,
    bg: RGB,
    fg: RGB,
}

impl DisplayTerminal<'_> {
    pub fn create<'a>(dp: GopDisplay<'a>, font: Arc<PSF>, bg: impl Color, fg: impl Color) -> DisplayTerminal<'a> {
        let (dp_width, dp_height) = dp.resolution();
        let (font_width, font_height) = (font.width, font.height);
        DisplayTerminal {
            dp: dp,
            font: font,
            buf: {
                let char_count = (dp_width / font_width as usize) * (dp_height / font_height as usize);
                let mut buf = Vec::with_capacity(char_count);
                for _ in 0..char_count {
                    buf.push(' ');
                }
                buf
            },
            bg: bg.into_rgb(),
            fg: fg.into_rgb(),
        }
    }

    fn get_index(&self, x: usize, y: usize) -> usize {
        self.width() * y + x
    }

    fn get_char(&self, x: usize, y: usize) -> char {
        let i = self.get_index(x, y);
        self.buf[i]
    }
}

impl Terminal for DisplayTerminal<'_> {
    fn resolution(&self) -> (usize, usize) {
        let width = self.dp.width() / self.font.width as usize;
        let height = self.dp.height() / self.font.height as usize;
        (width, height)
    }

    fn set_char(&mut self, x: usize, y: usize, c: char) {
        let i = self.get_index(x, y);
        self.buf.as_mut_slice()[i] = c;
    }

    fn clear(&mut self) {
        for x in 0..self.width() {
            for y in 0..self.height() {
                self.set_char(x, y, ' ');
            }
        }
    }

    fn refresh(&mut self) {
        self.dp.clear(self.bg);
        for x in 0..self.width() {
            for y in 0..self.height() {
                let glyph = self.font.lookup(self.get_char(x, y)).expect("Character missing from font.");
                unsafe {
                    self.dp.draw_glyph(self.fg, self.font.width as usize * x, self.font.height as usize * y, glyph);
                }
            }
        }
    }
}
