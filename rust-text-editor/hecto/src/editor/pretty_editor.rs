use std::io::{self, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

const CYAN: &str = "\x1b[36m";
const BLUE_BG: &str = "\x1b[44m";
const YELLOW_BG: &str = "\x1b[43m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";
const CLEAR: &str = "\x1b[2J\x1b[H";

pub struct PrettyEditor {
    rows: [&'static str; 4],
    cursor: usize,
}

impl PrettyEditor {
    pub fn new() -> Self {
        Self {
            rows: [
                "🌈 Welcome to Colorful Editor!",
                "Use 'j'/'k' to move | 'q' to quit",
                "Line numbers are cyan 💙",
                "Current line is highlighted 💡",
            ],
            cursor: 0,
        }
    }

    pub fn run(&mut self) {
        while self.draw().is_ok() && self.handle_input() {}
    }

    fn handle_input(&mut self) -> bool {
        let mut b = [0];
        io::stdin().read_exact(&mut b).ok()?;
        match b[0] {
            b'q' => return false,
            b'j' if self.cursor + 1 < self.rows.len() => self.cursor += 1,
            b'k' if self.cursor > 0 => self.cursor -= 1,
            _ => {}
        }
        true
    }

    fn draw(&self) -> io::Result<()> {
        let mut out = io::stdout();
        write!(out, "{CLEAR}")?;
        for (i, row) in self.rows.iter().enumerate() {
            let line_num = format!("{CYAN}{:>3}{RESET} ", i + 1);
            if i == self.cursor {
                write!(out, "{BLUE_BG}{DIM}{line_num}{row}{RESET}\r\n")?;
            } else {
                write!(out, "{DIM}{line_num}{row}{RESET}\r\n")?;
            }
        }
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        write!(
            out,
            "{YELLOW_BG} Line {} | {} lines | Time: {} {RESET}\r\n",
            self.cursor + 1,
            self.rows.len(),
            time
        )?;
        out.flush()
    }
}
