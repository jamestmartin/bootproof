use alloc::sync::Arc;
use crate::graphics::color::RGB;
use crate::graphics::display::Display;
use crate::graphics::font::psf::PSF;
use crate::graphics::terminal::Terminal;
use crate::graphics::terminal::frame::TerminalFrame;

pub struct DisplayTerminal<'display> {
    display: &'display mut dyn Display,
    frame: TerminalFrame,
    font: Arc<PSF>,
    bg: RGB,
    fg: RGB,
}

impl DisplayTerminal<'_> {
    pub fn new<'display>(display: &'display mut dyn Display, font: Arc<PSF>, bg: RGB, fg: RGB) -> DisplayTerminal<'display> {
        let (dp_width, dp_height) = display.resolution();
        let (ft_width, ft_height) = font.resolution();
        let ch_width = dp_width / ft_width as usize;
        let ch_height = dp_height / ft_height as usize;

        DisplayTerminal {
            display: display,
            frame: TerminalFrame::new((ch_width, ch_height)),
            font: font,
            bg: bg,
            fg: fg,
        }
    }
}

impl Terminal for DisplayTerminal<'_> {
    fn get_frame<'a>(&'a self) -> &'a TerminalFrame {
        &self.frame
    }

    fn borrow_frame<'a>(&'a mut self) -> &'a mut TerminalFrame {
        &mut self.frame
    }

    fn refresh(&mut self) {
        self.display.clear(self.bg);
        for x in 0..self.frame.width() {
            for y in 0..self.frame.height() {
                let c = self.frame.get(x, y);
                if c == '\u{0}' { continue; }

                let glyph = self.font.lookup(c).expect("Character missing from font.");
                unsafe {
                    self.display.draw_glyph(self.fg, self.font.width() as usize * x, self.font.height() as usize * y, glyph);
                }
            }
        }
    }
}
