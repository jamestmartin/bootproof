pub mod serial;
pub mod terminal;

use crate::graphics::tty::serial::SerialTty;

pub trait Tty {
    fn putc(&mut self, c: char);
    fn puts(&mut self, s: &str);
    fn clear(&mut self);
    fn flush(&mut self);
}

pub static mut STDOUT: Option<SerialTty> = None;
pub static mut STDERR: Option<SerialTty> = None;

// HACK: These macros are horribly repetitive. There's got to be a better way...

#[macro_export]
macro_rules! print {
    // These additional single-argument cases are necessary because `format!` requires allocation,
    // which I don't necessarily *have* (not to mention it's inefficient).
    ($s:expr) => {
        let mut tty;
        unsafe {
            tty = crate::graphics::tty::STDOUT.clone().unwrap();
        }
        tty.puts($s);
        tty.flush();
    };
    ($($arg:expr),*) => {
        let mut tty;
        unsafe {
            tty = crate::graphics::tty::STDOUT.clone().unwrap();
        }
        tty.puts(&alloc::format!($($arg),*));
        tty.flush();
    }
}

#[macro_export]
macro_rules! println {
    ($s:expr) => {
        let mut tty;
        unsafe {
            tty = crate::graphics::tty::STDOUT.clone().unwrap();
        }
        tty.puts($s);
        tty.putc('\n');
        tty.flush();
    };
    ($($arg:expr),*) => {
        let mut tty;
        unsafe {
            tty = crate::graphics::tty::STDOUT.clone().unwrap();
        }
        tty.puts(&alloc::format!($($arg),*));
        tty.putc('\n');
        tty.flush();
    }
}

#[macro_export]
macro_rules! eprint {
    ($s:expr) => {
        let mut tty;
        unsafe {
            tty = crate::graphics::tty::STDERR.clone().unwrap();
        }
        tty.puts($s);
        tty.flush();
    };
    ($($arg:expr),*) => {
        let mut tty;
        unsafe {
            tty = crate::graphics::tty::STDERR.clone().unwrap();
        }
        tty.puts(&alloc::format!($($arg),*));
        tty.flush();
    }
}

#[macro_export]
macro_rules! eprintln {
    ($s:expr) => {
        let mut tty;
        unsafe {
            tty = crate::graphics::tty::STDERR.clone().unwrap();
        }
        tty.puts($s);
        tty.putc('\n');
        tty.flush();
    };
    ($($arg:expr),*) => {
        let mut tty;
        unsafe {
            tty = crate::graphics::tty::STDERR.clone().unwrap();
        }
        tty.puts(&alloc::format!($($arg),*));
        tty.putc('\n');
        tty.flush();
    }
}

#[macro_export]
macro_rules! panic {
    ($($arg:expr),*) => {
        crate::eprintln!($($arg),*);
        crate::arch::x86_64::halt();
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic!("{}", info);
}
