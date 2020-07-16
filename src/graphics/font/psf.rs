use alloc::vec::Vec;

pub struct UnicodeMap {
    pub c: char,
    pub i: usize,
}

pub struct PSF {
    pub width: u32,
    pub height: u32,
    pub length: u32,
    pub charsize: u32,
    pub glyphs: Vec<u8>,
    pub unicode: Vec<UnicodeMap>,
}

pub struct PSFGlyph<'a> {
    width: u32,
    height: u32,
    bitmap: &'a [u8],
}

impl PSF {
    fn index_of(&self, c: char) -> Option<usize> {
        for entry in &self.unicode {
            if entry.c == c {
                return Some(entry.i);
            }
        }
        None
    }

    fn get_bitmap<'a>(&'a self, index: usize) -> &'a [u8] {
        let byte_index = self.charsize as usize * index;
        &self.glyphs[byte_index..byte_index + self.charsize as usize]
    }

    pub fn lookup<'a>(&'a self, c: char) -> Option<PSFGlyph<'a>> {
        self.index_of(c).map(|i| PSFGlyph {
            width: self.width,
            height: self.height,
            bitmap: self.get_bitmap(i)
        })
    }
}

impl PSFGlyph<'_> {
    pub fn width(&self) -> u32 { self.width }

    pub fn height(&self) -> u32 { self.height }

    pub fn get(&self, x: u32, y: u32) -> bool {
        let line_size = num_integer::div_ceil(self.width, 8);
        let char_size = line_size * self.height;
        let (line_byte_index, bit_index) = num_integer::div_rem(x, 8);
        let mask = 0b10000000 >> bit_index;
        let byte = self.bitmap[(y * line_size + line_byte_index) as usize];
        byte & mask > 0
    }
}
