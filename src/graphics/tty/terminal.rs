use alloc::string::{String, ToString};
use alloc::vec::Vec;
use crate::graphics::terminal::Terminal;
use crate::graphics::tty::Tty;

pub struct TerminalTty<'terminal> {
    term: &'terminal mut dyn Terminal,
    history: Vec<String>,
}

impl TerminalTty<'_> {
    pub fn new<'a>(term: &'a mut dyn Terminal) -> TerminalTty<'a> {
        TerminalTty {
            term: term,
            history: {
                let mut vec = Vec::new();
                vec.push("".to_string());
                vec
            },
        }
    }
}

impl Tty for TerminalTty<'_> {
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
        let mut physical_lines = Vec::new();
        for line in &self.history {
            let mut chars = line.chars().collect::<Vec<_>>().into_iter();
            while chars.len() > 0 {
                let mut physical_line = String::new();
                let width = chars.len().min(self.term.get_frame().width());
                for _ in 0..width {
                    physical_line.push(chars.next().unwrap());
                }
                physical_lines.push(physical_line);
            }
        }

        let mut y = physical_lines.len().min(self.term.get_frame().height() - 1);
        let frame = self.term.borrow_frame();
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
