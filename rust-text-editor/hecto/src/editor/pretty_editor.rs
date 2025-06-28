use std::io::{self, stdout, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PrettyEditor {
    should_quit: bool,
    rows: Vec<String>,
    status_message: String,
    cursor_row: usize,
}

impl PrettyEditor {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            rows: vec![
                "Welcome to Pretty Editor!".to_string(),
                "Now with cursor movement (j/k),".to_string(),
                "line highlighting, and a better status bar.".to_string(),
                "Try pressing j or k to move the highlight.".to_string(),
                "Press 'q' to quit.".to_string(),
            ],
            status_message: String::new(),
            cursor_row: 0,
        }
    }

    pub fn run(&mut self) {
        self.status_message = self.default_status();
        self.refresh_screen().unwrap();

        while !self.should_quit {
            self.process_keypress();
            self.refresh_screen().unwrap();
        }
    }

    fn default_status(&self) -> String {
        format!("Pretty Editor | {} lines", self.rows.len())
    }

    fn process_keypress(&mut self) {
        let mut buffer = [0; 1];
        if let Ok(_) = io::stdin().read_exact(&mut buffer) {
            match buffer[0] {
                b'q' => self.should_quit = true,
                b'j' => {
                    if self.cursor_row + 1 < self.rows.len() {
                        self.cursor_row += 1;
                    }
                }
                b'k' => {
                    if self.cursor_row > 0 {
                        self.cursor_row -= 1;
                    }
                }
                _ => {}
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), io::Error> {
        let mut stdout = stdout();
        Self::clear_screen();
        self.draw_rows();
        self.draw_status_bar();
        Self::move_cursor_to(0, 0);
        stdout.flush()
    }

    fn draw_rows(&self) {
        for (i, row) in self.rows.iter().enumerate() {
            let line_number = format!("{:>4} ", i + 1);

            if i == self.cursor_row {
                // Highlight current row
                print!("\x1b[7m\x1b[2m{}{}\x1b[0m\r\n", line_number, row);
            } else {
                print!("\x1b[2m{}\x1b[0m{}\r\n", line_number, row);
            }
        }
    }

    fn draw_status_bar(&self) {
        let status = &self.status_message;
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let cursor_info = format!("Line {}", self.cursor_row + 1);

        print!(
            "\x1b[7m{:<width$}\x1b[0m\r\n",
            format!(" {} | {} | Time: {}", status, cursor_info, time),
            width = 80
        );
    }

    fn clear_screen() {
        print!("\x1b[2J\x1b[H");
    }

    fn move_cursor_to(x: u16, y: u16) {
        print!("\x1b[{};{}H", y + 1, x + 1);
    }
}
