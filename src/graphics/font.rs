use alloc::sync::Arc;
use alloc::vec::Vec;
use psf::*;

mod psf;

static mut FONT: Option<Arc<PSF>> = None;

pub fn font() -> Arc<PSF> {
    unsafe {
        FONT.clone().unwrap_or_else(|| {
            let font = Arc::new(parse_font());
            FONT = Some(font.clone());
            font
        })
    }
}

fn pad(slice: &[u8]) -> [u8; 4] {
    if slice.len() == 1 {
        [slice[0], 0, 0, 0]
    } else if slice.len() == 2 {
        [slice[0], slice[1], 0, 0]
    } else if slice.len() == 3 {
        [slice[0], slice[1], slice[2], 0]
    } else if slice.len() == 4 {
        [slice[0], slice[1], slice[2], slice[3]]
    } else {
        crate::panic!("Bad character length {}", slice.len())
    }
}

fn parse_font() -> PSF {
    use core::convert::TryInto;
    let font = core::include_bytes!("font/cozette.psf");
    let length = u32::from_le_bytes(font[16..20].try_into().unwrap());
    let charsize = u32::from_le_bytes(font[20..24].try_into().unwrap());
    let height = u32::from_le_bytes(font[24..28].try_into().unwrap());
    let width = u32::from_le_bytes(font[28..32].try_into().unwrap());
    crate::log!("{} {} {}", width, height, charsize);

    let glyphs_size = (length * charsize) as usize;
    let mut glyphs = Vec::with_capacity(glyphs_size);
    glyphs.extend_from_slice(&font[32..glyphs_size + 32]);

    let mut unicode_map = Vec::new();
    let mut unicode_info = &font[glyphs_size + 32..];
    let mut glyph = 0;
    let mut i = 0;
    while i < unicode_info.len() {
        let mut nc = unicode_info[i];

        while nc != 0xFE && nc != 0xFF {
            let ch_bytes = nc.leading_ones().max(1) as usize;
            let st = core::str::from_utf8(&unicode_info[i..i + ch_bytes as usize]).expect("Invalid character");
            let ch = st.chars().next().unwrap();
            unicode_map.push(UnicodeMap { c: ch, i: glyph });
            i += ch_bytes;
            nc = unicode_info[i];
        }

        // Ignore multi-codepoint spellings of characters (for now).
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
        length: length,
        charsize: charsize,
        glyphs: glyphs,
        unicode: unicode_map,
    }
}
