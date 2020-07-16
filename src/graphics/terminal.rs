pub mod display;

pub trait Terminal {
    fn resolution(&self) -> (usize, usize);
    fn width(&self) -> usize { self.resolution().0 }
    fn height(&self) -> usize { self.resolution().1 }

    fn set_char(&mut self, x: usize, y: usize, c: char);
    fn clear(&mut self);

    fn refresh(&mut self);
}
