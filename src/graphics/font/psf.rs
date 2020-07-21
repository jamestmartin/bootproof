use alloc::vec::Vec;
use crate::graphics::font::{Font, Glyph};

pub struct PSF {
    width: usize,
    height: usize,
    glyphs: Vec<PSFGlyph>,
    // TODO: Replace this with a proper associative structure.
    unicode: Vec<UnicodeMap>,
}

/// Associates a unicode character and a glyph.
struct UnicodeMap {
    c: char,
    // The index of the glyph.
    i: usize,
}

pub struct PSFGlyph {
    bitmap: Vec<u8>,
    line_size: usize,
    width: usize,
    height: usize,
}

impl PSF {
    pub fn parse(font: &[u8]) -> PSF {
        use core::convert::TryInto;

        // The number of glyphs in this font.
        let length = u32::from_le_bytes(font[16..20].try_into().unwrap()) as usize;
        // The size in bytes of a single glyph.
        let charsize = u32::from_le_bytes(font[20..24].try_into().unwrap()) as usize;
        // The height in pixels of this font's bounding box.
        let height = u32::from_le_bytes(font[24..28].try_into().unwrap()) as usize;
        // The width in pixels of this font's bounding box.
        let width = u32::from_le_bytes(font[28..32].try_into().unwrap()) as usize;
        // The size in bytes of a single row of pixels in a glyph.
        let line_size = num_integer::div_ceil(width, 8);

        let glyphs_offset = 32; // the size of the header
        let glyphs_size = length * charsize;
        let unicode_offset = glyphs_offset + glyphs_size;

        let mut glyphs = Vec::with_capacity(length);

        for i in 0..length {
            let mut bitmap = Vec::with_capacity(charsize);
            let bitmap_begin = glyphs_offset + charsize * i;
            let bitmap_end = bitmap_begin + charsize;
            bitmap.extend_from_slice(&font[bitmap_begin..bitmap_end]);

            glyphs.push(PSFGlyph {
                bitmap: bitmap,
                line_size: line_size,
                // Glyphs may overflow the font's nominal resolution in the padding bytes of the line!
                // This trick only works for the width because there is no vertical padding.
                // TODO: Pre-compute widths and bounding box offsets of individual glyphs.
                width: line_size * 8,
                height: height,
            });
        }

        // HACK: This unicode map parser is still a mess.
        let mut unicode_map = Vec::new();
        let unicode_info = &font[unicode_offset..];
        let mut glyph = 0;
        let mut i = 0;
        while i < unicode_info.len() {
            let mut nc = unicode_info[i];

            while nc != 0xFE && nc != 0xFF {
                let ch_bytes = nc.leading_ones().max(1) as usize;
                let st = core::str::from_utf8(&unicode_info[i..i + ch_bytes]).expect("Invalid character");
                let ch = st.chars().next().unwrap();
                unicode_map.push(UnicodeMap { c: ch, i: glyph });
                i += ch_bytes;
                nc = unicode_info[i];
            }

            // TODO: Support multi-codepoint spellings of characters.
            while nc != 0xFF {
                i += 1;
                nc = unicode_info[i];
            }

            i += 1;
            glyph += 1;
        }

        PSF {
            width: width,
            height: height,
            glyphs: glyphs,
            unicode: unicode_map,
        }
    }

    /// The index of the glyph associated with a particular unicde character.
    fn index_of(&self, c: char) -> Option<usize> {
        for entry in &self.unicode {
            if entry.c == c {
                return Some(entry.i);
            }
        }
        None
    }
}

impl Font for PSF {
    type Glyph = PSFGlyph;

    fn bounding_box(&self) -> (usize, usize) {
        (self.width, self.height)
    }

    fn lookup<'a>(&'a self, c: char) -> Option<&'a PSFGlyph> {
        self.index_of(c).map(|i| &self.glyphs[i])
    }
}

impl Glyph for PSFGlyph {
    fn width(&self) -> usize { self.width }
    fn height(&self) -> usize { self.height }

    fn get(&self, x: usize, y: usize) -> bool {
        if x > self.width || y > self.height {
            crate::panic!("Glyph pixel index out of bounds.");
        }

        let (line_byte_index, bit_index) = num_integer::div_rem(x, 8);
        let mask = 0b10000000 >> bit_index;
        let byte = self.bitmap[(y * self.line_size + line_byte_index) as usize];
        byte & mask > 0
    }
}
