pub mod serial;
pub mod text_display;

/// A teletypewriter, or really, because those don't exist anymore,
/// a device that behaves like or emulates a teletypewriter.
/// Basically, this is a device that lets you output text and not much else.
/// Its output may be buffered, so make sure you `flush` the output.
pub trait Tty {
    /// Print a single character to the TTY.
    fn putc(&mut self, c: char);
    /// Print an entire string to the TTY.
    fn puts(&mut self, s: &str);
    /// Clear all TTY output.
    fn clear(&mut self);
    /// Synchronously flush any buffered output.
    fn flush(&mut self);
}
