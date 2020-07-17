pub mod serial;
pub mod terminal;
pub mod uefi;

use crate::graphics::tty::serial::SerialTty;

pub trait Tty {
    fn putc(&mut self, c: char);
    fn puts(&mut self, s: &str);
    fn clear(&mut self);
    fn flush(&mut self);
}

pub static mut STDOUT: Option<SerialTty> = None;
pub static mut STDERR: Option<SerialTty> = None;

#[macro_export]
macro_rules! print {
    ($( $arg:expr ),* ) => {
        let mut tty;
        unsafe {
            tty = crate::graphics::tty::STDOUT.clone().unwrap();
        }
        tty.puts(&alloc::format!($( $arg ),*));
        tty.flush();
    }
}

#[macro_export]
macro_rules! println {
    ($( $arg:expr ),* ) => {
        let mut tty;
        unsafe {
            tty = crate::graphics::tty::STDOUT.clone().unwrap();
        }
        tty.puts(&alloc::format!($( $arg ),*));
        tty.putc('\n');
        tty.flush();
    }
}

#[macro_export]
macro_rules! eprint {
    ($( $arg:expr ),* ) => {
        let mut tty;
        unsafe {
            tty = crate::graphics::tty::STDOUT.clone().unwrap();
        }
        tty.puts(&alloc::format!($( $arg ),*));
        tty.flush();
    }
}

#[macro_export]
macro_rules! eprintln {
    ($( $arg:expr ),* ) => {
        let mut tty;
        unsafe {
            tty = crate::graphics::tty::STDOUT.clone().unwrap();
        }
        tty.puts(&alloc::format!($( $arg ),*));
        tty.putc('\n');
        tty.flush();
    }
}

#[macro_export]
macro_rules! panic {
    ($( $arg:expr ),* ) => {
        crate::eprintln!($( $arg ),*);
        crate::misc::halt()
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic!("{}", info);
}
