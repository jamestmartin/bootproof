pub mod color;
pub mod display;
pub mod font;
pub mod terminal;
#[macro_use]
pub mod tty;

use crate::graphics::color::{COLOR_BLACK, COLOR_WHITE};
use crate::graphics::display::gop::GopDisplay;
use crate::graphics::font::psf::PSF;
use crate::graphics::terminal::display::DisplayTerminal;
use crate::graphics::tty::Tty;
use crate::graphics::tty::terminal::TerminalTty;

pub fn do_graphics(st: &uefi::prelude::SystemTable<uefi::prelude::Boot>) {
    let font_data = core::include_bytes!("graphics/font/cozette.psf");
    let font = PSF::parse(font_data);

    let mut display = GopDisplay::init(st.boot_services());
    let mut terminal = DisplayTerminal::new(&mut display, &font, COLOR_BLACK, COLOR_WHITE);
    let mut tty = TerminalTty::new(&mut terminal);

    for _ in 0..30 {
        for c in 'a'..'z' {
            tty.putc(c);
            tty.putc('\n');
        }
    }

    for _ in 0..20 {
        for c in 'a'..'z' {
            tty.putc(c);
        }
    }
    tty.putc('\n');
    tty.puts("✔ Hello, world! ♡");
    tty.flush();
}
