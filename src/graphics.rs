pub mod color;
pub mod display;
pub mod font;
pub mod terminal;
pub mod tty;

use crate::graphics::color::{COLOR_BLACK, COLOR_WHITE};
use crate::graphics::display::gop::GopDisplay;
use crate::graphics::font::font;
use crate::graphics::terminal::display::DisplayTerminal;
use crate::graphics::tty::Tty;
use crate::graphics::tty::terminal::TerminalTty;

pub fn do_graphics(st: &uefi::prelude::SystemTable<uefi::prelude::Boot>) {
    let display = GopDisplay::init(st.boot_services());
    let terminal = DisplayTerminal::create(display, font(), COLOR_BLACK, COLOR_WHITE);
    let mut tty = TerminalTty::create(terminal);

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
