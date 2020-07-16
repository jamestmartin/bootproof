use core::fmt::Write;

pub enum LoggerBackend {
    None,
    // It is impossible to get ownership of an Output,
    // so instead we must pass in the entire boot system table.
    UefiStdio(uefi::prelude::SystemTable<uefi::prelude::Boot>),
}

enum LoggerOutput {
    Stdout,
    Stderr
}

pub struct Logger<'a> {
    backend: &'a LoggerBackend,
    output: LoggerOutput
}

impl LoggerBackend {
    pub fn stdout<'a>(&'a self) -> Logger<'a> {
        Logger {
            backend: self,
            output: LoggerOutput::Stdout
        }
    }

    pub fn stderr<'a>(&'a self) -> Logger<'a> {
        Logger {
            backend: self,
            output: LoggerOutput::Stderr
        }
    }
}

impl Write for Logger<'_> {
    fn write_str(&mut self, s: &str) -> Result<(), core::fmt::Error> {
        match self.backend {
            LoggerBackend::UefiStdio(st) => {
                let output = match &self.output {
                    LoggerOutput::Stdout => st.stdout(),
                    LoggerOutput::Stderr => st.stderr()
                };
                output.write_str(s)
            },
            LoggerBackend::None => {
                // There's pretty much no way to recover from a missing logger.
                // What are we supposed to do-- log the error?
                Ok(())
            },
        }
    }
}

pub static mut LOGGER_BACKEND: LoggerBackend = LoggerBackend::None;

#[macro_export]
macro_rules! log {
    ($( $arg:expr ),* ) => {
        unsafe {
            use core::fmt::Write;
            core::writeln!(crate::logger::LOGGER_BACKEND.stderr(), $( $arg ),*).unwrap();
        }
    }
}

#[macro_export]
macro_rules! print {
    ($( $arg:expr ),* ) => {
        unsafe {
            use core::fmt::Write;
            core::write!(crate::logger::LOGGER_BACKEND.stdout(), $( $arg ),*).unwrap();
        }
    }
}

#[macro_export]
macro_rules! println {
    ($( $arg:expr ),* ) => {
        unsafe {
            use core::fmt::Write;
            core::writeln!(crate::logger::LOGGER_BACKEND.stdout(), $( $arg ),*).unwrap();
        }
    }
}

#[macro_export]
macro_rules! panic {
    ($( $arg:expr ),* ) => {
        {
            use crate::misc::halt;
            crate::log!($( $arg ),*);
            halt()
        }
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    panic!("{}", info);
}
