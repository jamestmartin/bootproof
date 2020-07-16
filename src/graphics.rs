mod font;

use crate::println;
use uefi::proto::console::gop::*;

fn px_index(stride: usize, x: usize, y: usize) -> usize {
    4 * (y * stride + x)
}

fn make_px(r: u8, g: u8, b: u8) -> [u8; 4] {
    [b, g, r, 0]
}

unsafe fn draw_char(fb: &mut FrameBuffer, stride: usize, cx: usize, cy: usize, c: char) {
    let font = font::font();
    let glyph = font.lookup(c).expect("Character missing from font.");
    let color = make_px(255, 255, 255);
    for dx in 0..num_integer::div_ceil(glyph.width(), 8) * 8 {
        for dy in 0..glyph.height() {
            if glyph.get(dx, dy) {
                let scale = 2;
                for sdx in 0..scale {
                    for sdy in 0..scale {
                        let px = cx + scale * dx as usize + sdx;
                        let py = cy + scale * dy as usize + sdy;
                        fb.write_value(px_index(stride, px, py), color);
                    }
                }
            }
        }
    }
}

unsafe fn draw_str(fb: &mut FrameBuffer, stride: usize, mut cx: usize, cy: usize, text: &str) {
    let width = font::font().width as usize * 2;
    for c in text.chars() {
        draw_char(fb, stride, cx, cy, c);
        cx += width;
    }
}

fn draw(fb: &mut FrameBuffer, stride: usize, width: usize, height: usize) {
    for x in 0..width {
        for y in 0..height {
            let i = px_index(stride, x, y);
            let r = (x * 256 / width) as u8;
            let g = (y * 256 / height) as u8;
            let b = 255 - ((r as u16 + g as u16) / 2) as u8;
            let px = make_px(r, g, b);
            unsafe {
                fb.write_value(i, px);
            }
        }
    }

    let c_width = width / 8;
    let c_height = height / 16;
    unsafe {
        draw_str(fb, stride, 8, 8, "✔ Hello, world! ♡");
    }
}

pub fn do_graphics(st: &uefi::prelude::SystemTable<uefi::prelude::Boot>) {
    let gop = st.boot_services().locate_protocol::<GraphicsOutput>()
        .unwrap()
        .expect("UEFI Graphics Output Protocol (GOP) is not present.");
    let mut gop = unsafe { &mut *gop.get() };
    let mut mode = None;
    for gop_mode in gop.modes() {
        let gop_mode = gop_mode.expect("Warning while accessing GOP mode.");
        if let PixelFormat::BGR = gop_mode.info().pixel_format() {
            mode = Some(gop_mode);
        } else {
            println!("Ignoring non-BGR pixel format.");
        }
    }
    let mode = mode.expect("No usable pixel formats found.");
    let (width, height) = mode.info().resolution();
    let stride = mode.info().stride();
    println!("Using mode: {}x{} {:?}", width, height, mode.info().pixel_format());
    gop.set_mode(&mode).unwrap().expect("Failed to set UEFI Graphics Output mode.");
    let mut fb = gop.frame_buffer();

    draw(&mut fb, stride, width, height);
}
