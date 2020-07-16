use crate::graphics::color::Color;
use crate::graphics::display::Display;
use uefi::proto::console::gop::{FrameBuffer, GraphicsOutput, ModeInfo, PixelFormat};

const PIXEL_WIDTH_BYTES: usize = 4;

pub struct GopDisplay<'a> {
    fb: FrameBuffer<'a>,
    mode: ModeInfo,
}

impl GopDisplay<'_> {
    pub fn init<'boot>(bs: &'boot uefi::table::boot::BootServices) -> GopDisplay<'boot> {
        let gop = bs.locate_protocol::<GraphicsOutput>()
            .expect("UEFI Graphics Output Protocol (GOP) is not present.")
            .unwrap();
        let gop = unsafe { &mut *gop.get() };
        let mut mode = None;
        for gop_mode in gop.modes() {
            let gop_mode = gop_mode.unwrap();
            if let PixelFormat::BGR = gop_mode.info().pixel_format() {
                mode = Some(gop_mode);
            }
        }
        let mode = mode.expect("No usable pixel formats found.");
        let (width, height) = mode.info().resolution();
        crate::log!("Using mode: {}x{} {:?}", width, height, mode.info().pixel_format());
        gop.set_mode(&mode).expect("Failed to set UEFI Graphics Output mode.").unwrap();

        let info = gop.current_mode_info();
        GopDisplay {
            fb: gop.frame_buffer(),
            mode: info,
        }
    }

    // Convert a color to a BGR-formatted byte array.
    fn make_pixel(&self, color: impl Color) -> [u8; 4] {
        [color.b(), color.g(), color.r(), 0]
    }

    fn pixel_index(&self, x: usize, y: usize) -> usize {
        PIXEL_WIDTH_BYTES * (self.mode.stride() * y + x)
    }
}

impl Display for GopDisplay<'_> {
    fn resolution(&self) -> (usize, usize) { self.mode.resolution() }

    unsafe fn set_pixel(&mut self, color: impl Color, x: usize, y: usize) {
        self.fb.write_value(self.pixel_index(x, y), self.make_pixel(color));
    }

    fn clear(&mut self, color: impl Color) {
        let (width, height) = self.resolution();
        let px = self.make_pixel(color);
        for x in 0..width {
            for y in 0..height {
                unsafe {
                    self.fb.write_value(self.pixel_index(x, y), px);
                }
            }
        }
    }
}
