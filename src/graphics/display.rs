use crate::graphics::color::RGB;
use crate::graphics::font::psf::PSFGlyph;

pub mod gop;

pub trait Display {
    fn resolution(&self) -> (usize, usize);
    fn width(&self) -> usize { self.resolution().0 }
    fn height(&self) -> usize { self.resolution().1 }

    unsafe fn set_pixel(&mut self, color: RGB, x: usize, y: usize);
    fn set_pixel_ignore_oob(&mut self, color: RGB, x: usize, y: usize) {
        if x > self.width() || y > self.height() {
            return;
        }

        unsafe {
            self.set_pixel(color, x, y);
        }
    }

    fn clear(&mut self, color: RGB);

    unsafe fn draw_glyph(&mut self, color: RGB, x: usize, y: usize, glyph: PSFGlyph) {
        // Glyphs may actually be larger than their nominal bounding box.
        // In fact, the Cozette font is like this: the heart symbol is 7 pixels wide,
        // despite nominally being a 6x13 font.
        // However, despite not being an intended use of the format, that extra pixel
        // can still be stored in the padding bits of the glyph (and is!).
        // Therefore, we just continue writing those extra bits if they are present.
        // Note that there is no similar trick for the height,
        // because the height doesn't have padding.
        for glyph_x in 0..glyph.width() {
            for glyph_y in 0..glyph.height() {
                if glyph.get(glyph_x, glyph_y) {
                    self.set_pixel(color, x + glyph_x as usize, y + glyph_y as usize);
                }
            }
        }

        // Sometimes, a font may actually have pixels outside its bounding box!
        // For example, in Cozette, a 6x13 font, ♡ is actually 7 pixels wide.
        // This data is still stored in the padding bits of the glyph.
        // Note that there is no similar trick for height because height doesn't have padding.
        // Futhermore, this only works on fonts whose width is not a multiple of eight.
        for glyph_x in glyph.width()..num_integer::div_ceil(glyph.width(), 8) * 8 {
            for glyph_y in 0..glyph.height() {
                if glyph.get(glyph_x, glyph_y) {
                    // These pixels *nominally* aren't supposed to be there,
                    // so we only force the pixels inside the bounding box.
                    self.set_pixel_ignore_oob(color, x + glyph_x as usize, y + glyph_y as usize);
                }
            }
        }
    }
}
