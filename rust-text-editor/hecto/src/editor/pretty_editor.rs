use std::io::{self, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PrettyEditor {
    rows: Vec<String>,
    cursor: usize,
}

impl PrettyEditor {
    pub fn new() -> Self {
        Self {
            rows: vec![
                "Welcome to Pretty Editor!".into(),
                "Use 'j'/'k' to move, 'q' to quit.".into(),
                "This is a compact version.".into(),
            ],
            cursor: 0,
        }
    }

    pub fn run(&mut self) {
        while self.draw().is_ok() && self.input() {}
    }

    fn input(&mut self) -> bool {
        let mut b = [0]; io::stdin().read_exact(&mut b).ok()?;
        match b[0] {
            b'q' => false,
            b'j' if self.cursor + 1 < self.rows.len() => self.cursor += 1,
            b'k' if self.cursor > 0 => self.cursor -= 1,
            _ => {}
        }
        true
    }

    fn draw(&self) -> io::Result<()> {
        print!("\x1b[2J\x1b[H"); // clear screen
        for (i, r) in self.rows.iter().enumerate() {
            let num = format!("{:>3} ", i + 1);
            let row = if i == self.cursor {
                format!("\x1b[7m\x1b[2m{}{}\x1b[0m", num, r)
            } else {
                format!("\x1b[2m{}{}\x1b[0m", num, r)
            };
            println!("{}", row);
        }
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        println!("\x1b[7m Line {} | {} lines | Time: {} \x1b[0m", self.cursor + 1, self.rows.len(), time);
        io::stdout().flush()
    }
}
