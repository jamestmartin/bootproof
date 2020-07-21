use crate::driver::graphic_display::GraphicDisplay;
use crate::driver::text_display::{TextDisplayFrame, TextDisplay};
use crate::graphics::color::{Color, RGB};
use crate::graphics::font::{Font, Glyph};

/// A virtual text display that renders itself onto a graphic display.
pub struct GraphicTextDisplay<'d, 'f, G: Glyph> {
    display: &'d mut (dyn GraphicDisplay + 'd),
    font: &'f (dyn Font<Glyph = G> + 'f),
    frame: TextDisplayFrame,
    bg: RGB,
    fg: RGB,
}

impl<G: Glyph> GraphicTextDisplay<'_, '_, G> {
    pub fn new<'d, 'f>
            (display: &'d mut (dyn GraphicDisplay + 'd), font: &'f (dyn Font<Glyph = G> + 'f),
             bg: impl Color, fg: impl Color)
            -> GraphicTextDisplay<'d, 'f, G> {
        let (dp_width, dp_height) = display.resolution();
        let (ft_width, ft_height) = font.bounding_box();
        let ch_width = dp_width / ft_width as usize;
        let ch_height = dp_height / ft_height as usize;

        GraphicTextDisplay {
            display: display,
            font: font,
            frame: TextDisplayFrame::new((ch_width, ch_height)),
            bg: bg.into_rgb(),
            fg: fg.into_rgb(),
        }
    }
}

impl<G: Glyph> TextDisplay for GraphicTextDisplay<'_, '_, G> {
    fn borrow_frame<'a>(&'a self) -> &'a TextDisplayFrame {
        &self.frame
    }

    fn borrow_mut_frame<'a>(&'a mut self) -> &'a mut TextDisplayFrame {
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
