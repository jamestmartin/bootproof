pub mod serial;
pub mod terminal;

pub trait Tty {
    fn putc(&mut self, c: char);
    fn puts(&mut self, s: &str);
    fn clear(&mut self);
    fn flush(&mut self);
}
