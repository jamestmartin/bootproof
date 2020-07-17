pub mod display;
pub mod frame;

use crate::graphics::terminal::frame::TerminalFrame;

pub trait Terminal {
    fn get_frame<'a>(&'a self) -> &'a TerminalFrame;
    fn borrow_frame<'a>(&'a mut self) -> &'a mut TerminalFrame;
    fn refresh(&mut self);
}
