use crate::graphics::color::{Color, RGB};
use crate::graphics::display::Display;
use crate::graphics::font::{Font, Glyph};
use crate::graphics::terminal::Terminal;
use crate::graphics::terminal::frame::TerminalFrame;

pub struct DisplayTerminal<'d, 'f, G: Glyph> {
    display: &'d mut (dyn Display + 'd),
    font: &'f (dyn Font<Glyph = G> + 'f),
    frame: TerminalFrame,
    bg: RGB,
    fg: RGB,
}

impl<G: Glyph> DisplayTerminal<'_, '_, G> {
    pub fn new<'d, 'f>
            (display: &'d mut (dyn Display + 'd), font: &'f (dyn Font<Glyph = G> + 'f),
             bg: impl Color, fg: impl Color)
            -> DisplayTerminal<'d, 'f, G> {
        let (dp_width, dp_height) = display.resolution();
        let (ft_width, ft_height) = font.bounding_box();
        let ch_width = dp_width / ft_width as usize;
        let ch_height = dp_height / ft_height as usize;

        DisplayTerminal {
            display: display,
            font: font,
            frame: TerminalFrame::new((ch_width, ch_height)),
            bg: bg.into_rgb(),
            fg: fg.into_rgb(),
        }
    }
}

impl<G: Glyph> Terminal for DisplayTerminal<'_, '_, G> {
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

                // FIXME: This code shouldn't throw errors.
                //   instead, it should display some kind of missing character.
                let glyph = self.font.lookup(c).expect("Character missing from font.");
                let px_x = x * self.font.bounding_box().0;
                let px_y = y * self.font.bounding_box().1;
                unsafe {
                    self.display.draw_glyph(self.font.bounding_box(), px_x, px_y, self.fg, glyph);
                }
            }
        }
    }
}
