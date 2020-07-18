use crate::graphics::color::RGB;
use crate::graphics::font::Glyph;

pub trait Display {
    fn resolution(&self) -> (usize, usize);
    fn width(&self) -> usize { self.resolution().0 }
    fn height(&self) -> usize { self.resolution().1 }

    // HACK: This interface sucks.
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

    unsafe fn draw_glyph(&mut self, bounding_box: (usize, usize), x: usize, y: usize, color: RGB, glyph: &dyn Glyph) {
        // We only assume that space was left for pixels within the bounding box,
        // and that pixels outside the bounding box may be out-of-bounds.
        // We use `set_pixel` for the in-bounds pixels and `set_pixel_ignore_oob`
        // for the out-of-bounds pixels.

        // HACK: I should be able to figure out whether a row or column will be out-of-bounds statically, and:
        //   1. If it is going to be out-of-bounds and is inside the bounding box, panic, and
        //   2. if it is outside of the bounding box, don't bother trying to draw that row.

        for glyph_x in 0..glyph.width().min(bounding_box.0) {
            for glyph_y in 0..glyph.height().min(bounding_box.1) {
                if glyph.get(glyph_x, glyph_y) {
                    self.set_pixel(color, x + glyph_x, y + glyph_y);
                }
            }
        }

        for glyph_x in glyph.width().min(bounding_box.0)..glyph.width() {
            for glyph_y in 0..glyph.height().min(bounding_box.1) {
                if glyph.get(glyph_x, glyph_y) {
                    // These pixels *nominally* aren't supposed to be there,
                    // so we only force the pixels inside the bounding box.
                    self.set_pixel_ignore_oob(color, x + glyph_x, y + glyph_y);
                }
            }
        }
    }
}
