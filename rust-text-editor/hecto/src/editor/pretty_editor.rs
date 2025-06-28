use std::io::{self, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const CYAN: &str = "\x1b[36m";
const BLUE_BG: &str = "\x1b[44m";
const YELLOW_BG: &str = "\x1b[43m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";

pub struct PrettyEditor {
    rows: Vec<String>,
    cursor: usize,
}

impl PrettyEditor {
    pub fn new() -> Self {
        Self {
            rows: vec![
                "🌈 Welcome to Colorful Editor!".into(),
                "Use 'j'/'k' to move | 'q' to quit".into(),
                "Line numbers are cyan 💙".into(),
                "Current line is highlighted 💡".into(),
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
            b'q' => return false,
            b'j' if self.cursor + 1 < self.rows.len() => self.cursor += 1,
            b'k' if self.cursor > 0 => self.cursor -= 1,
            _ => {}
        }
        true
    }

    fn draw(&self) -> io::Result<()> {
        print!("\x1b[2J\x1b[H"); // clear screen
        for (i, r) in self.rows.iter().enumerate() {
            let num = format!("{}{:>3}{} ", CYAN, i + 1, RESET);
            if i == self.cursor {
                println!("{}{}{}{}{}", BLUE_BG, DIM, num, r, RESET);
            } else {
                println!("{}{}{}", DIM, num, r);
            }
        }
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        println!(
            "{} Line {} | {} lines | Time: {} {}",
            YELLOW_BG,
            self.cursor + 1,
            self.rows.len(),
            time,
            RESET
        );
        io::stdout().flush()
    }
}
