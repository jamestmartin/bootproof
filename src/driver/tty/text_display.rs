use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::driver::text_display::TextDisplay;
use crate::driver::tty::Tty;

/// A buffered virtual TTY implemented over a textual display.
pub struct TextDisplayTty<'display> {
    term: &'display mut (dyn TextDisplay + 'display),
    history: Vec<String>,
}

impl TextDisplayTty<'_> {
    pub fn new<'a>(term: &'a mut dyn TextDisplay) -> TextDisplayTty<'a> {
        TextDisplayTty {
            term,
            history: {
                let mut vec = Vec::new();
                vec.push("".to_string());
                vec
            },
        }
    }
}

impl Tty for TextDisplayTty<'_> {
    fn putc(&mut self, c: char) {
        if c == '\n' {
            self.history.push("".to_string());
            return;
        }
        let i = self.history.len() - 1;
        self.history[i].push(c);
    }

    fn puts(&mut self, s: &str) {
        for c in s.chars() {
            self.putc(c);
        }
    }

    fn clear(&mut self) {
        self.history.clear();
        self.history.push("".to_string());
    }

    fn flush(&mut self) {
        // Each line of the history represents a virtual line of output.
        // However, a line of output may be longer than the physical width of the display,
        // in which case we may need to wrap the line so that it takes up two physical lines.
        let mut physical_lines = Vec::new();
        for line in &self.history {
            let mut chars = line.chars().collect::<Vec<_>>().into_iter();
            // We iterate over all of the characters in a virtual line
            // until every character has been added to a physical line.
            // It is necessary that we iterate at least once, or empty lines will not be printed.
            loop {
                let mut physical_line = String::new();
                // The width of a physical line may be no more than the width of the frame.
                let width = chars.len().min(self.term.borrow_frame().width());
                for _ in 0..width {
                    physical_line.push(chars.next().unwrap());
                }
                physical_lines.push(physical_line);

                if chars.len() == 0 {
                    break;
                }
            }
        }

        // This is how many lines on the display we'll need for all of our physical lines.
        // We cannot have more lines than allowed by the display.
        let mut y = physical_lines.len().min(self.term.borrow_frame().height()) - 1;
        let frame = self.term.borrow_mut_frame();
        // We start from the lowest line and display each line until we reach the top of the screen.
        // We cannot run out of physical lines because the lowest line
        // is at lowest the number of physical lines necessary to display all lines.
        for line in physical_lines.into_iter().rev() {
            let mut x = 0;
            for c in line.chars() {
                frame.set(x, y, c);
                x += 1;
            }

            if y == 0 {
                break;
            }

            y -= 1;
        }

        self.term.refresh();
    }
}
