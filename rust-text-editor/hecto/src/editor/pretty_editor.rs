use std::io::{self, stdout, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PrettyEditor {
    quit: bool,
    rows: Vec<String>,
    cursor: usize,
}

impl PrettyEditor {
    pub fn new() -> Self {
        Self {
            quit: false,
            rows: vec![
                "Welcome to Pretty Editor!".into(),
                "Use 'j'/'k' to move, 'q' to quit.".into(),
                "Current line is highlighted.".into(),
                "Feel free to modify this!".into(),
            ],
            cursor: 0,
        }
    }

    pub fn run(&mut self) {
        while !self.quit {
            self.refresh().unwrap();
            self.keypress();
        }
    }

    fn keypress(&mut self) {
        let mut buf = [0; 1];
        if io::stdin().read_exact(&mut buf).is_ok() {
            match buf[0] {
                b'q' => self.quit = true,
                b'j' if self.cursor + 1 < self.rows.len() => self.cursor += 1,
                b'k' if self.cursor > 0 => self.cursor -= 1,
                _ => {}
            }
        }
    }

    fn refresh(&self) -> io::Result<()> {
        let mut out = stdout();
        print!("\x1b[2J\x1b[H"); // clear screen
        for (i, row) in self.rows.iter().enumerate() {
            let line = format!("{:>3} ", i + 1);
            if i == self.cursor {
                print!("\x1b[7m\x1b[2m{}{}\x1b[0m\r\n", line, row);
            } else {
                print!("\x1b[2m{}\x1b[0m{}\r\n", line, row);
            }
        }

        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        print!(
            "\x1b[7m Line {} | {} lines | Time: {} \x1b[0m\r\n",
            self.cursor + 1,
            self.rows.len(),
            time
        );

        out.flush()
    }
}
