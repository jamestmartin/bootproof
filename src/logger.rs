use alloc::format;
use core::cell::UnsafeCell;
use crate::graphics::tty::Tty;
use crate::graphics::tty::serial::SerialTty;
use log::{Record, LevelFilter, Metadata, SetLoggerError};

enum GlobalLogger {
    None,
    // Hardcoding as SerialTty for now.
    // I can worry about dealing with other implementation types when necessary.
    Tty(UnsafeCell<SerialTty>),
}

use GlobalLogger::*;

impl log::Log for GlobalLogger {
    fn enabled(&self, _metadata: &Metadata) -> bool {
        match self {
            None => false,
            _ => true
        }
    }

    fn log(&self, record: &Record) {
        match self {
            None => {},
            Tty(tty) => unsafe {
                // TODO: Lose the dependency on the `format!` macro
                // so we don't have to allocate a String here.
                (*tty.get()).puts(&format!("{} - {}", record.level(), record.args()));
            },
        }
    }

    fn flush(&self) {
        match self {
            None => {},
            Tty(tty) => unsafe {
                (*tty.get()).flush();
            },
        }
    }
}

// The logger is not thread-safe, but for now we only use one processor.
// FIXME: Support multiple processors.
unsafe impl Sync for GlobalLogger {}
unsafe impl Send for GlobalLogger {}

static mut LOGGER: GlobalLogger = GlobalLogger::None;

pub fn init() -> Result<(), SetLoggerError> {
    unsafe {
        log::set_logger(&LOGGER)
            .map(|()| log::set_max_level(LevelFilter::Info))
    }
}

pub fn set_tty(tty: SerialTty) {
    unsafe {
        LOGGER = GlobalLogger::Tty(UnsafeCell::new(tty));
    }
}
