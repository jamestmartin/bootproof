#[derive(Copy, Clone)]
pub struct RGB {
    r: u8,
    g: u8,
    b: u8
}

pub trait Color: Copy {
    fn r(&self) -> u8;
    fn g(&self) -> u8;
    fn b(&self) -> u8;

    fn into_rgb(&self) -> RGB {
        RGB {
            r: self.r(),
            g: self.g(),
            b: self.b()
        }
    }
}

impl Color for RGB {
    fn r(&self) -> u8 { self.r }
    fn g(&self) -> u8 { self.g }
    fn b(&self) -> u8 { self.b }
}

pub const COLOR_BLACK: RGB = RGB { r: 0x23, g: 0x23, b: 0x23 };
pub const COLOR_WHITE: RGB = RGB { r: 0xFF, g: 0xFF, b: 0xFF };
