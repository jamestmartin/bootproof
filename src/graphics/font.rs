pub mod psf;

// Note that currently the Font and Glyph traits are fairly specialized to PSF.
// They will certainly have to be modified to support other types of fonts,
// but they *work* for now, and that's what's important/

pub trait Font {
    // Once Rust supports existential types, this needs to be an existential type.
    type Glyph: Glyph;

    /// The size, in pixels, of the bounding box of each glyph in this font.
    fn bounding_box(&self) -> (usize, usize);

    fn lookup<'a>(&'a self, ch: char) -> Option<&'a Self::Glyph>;
}

pub trait Glyph {
    /// The width, in pixels, of this specific glyph.
    /// This may be a different size than the font's bounding box.
    fn width(&self) -> usize;

    /// The height, in pixels, of this specific glyph.
    /// This may be a different size than the font's bounding box.
    fn height(&self) -> usize;

    // TODO: Support glyph offsets relative to the font bounding box.

    /// Check whether an individual pixel of this glyph is set.
    /// This function will panic if `x` and `y` are outside the width and height of this glyph.
    fn get(&self, x: usize, y: usize) -> bool;
}
