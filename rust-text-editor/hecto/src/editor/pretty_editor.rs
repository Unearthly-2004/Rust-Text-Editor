use std::io::{self, stdout, Write};
use std::time::{SystemTime, UNIX_EPOCH};

pub struct PrettyEditor {
    should_quit: bool,
    rows: Vec<String>,
    status_message: String,
}

impl PrettyEditor {
    pub fn new() -> Self {
        Self {
            should_quit: false,
            rows: vec![
                "Welcome to Pretty Editor!".to_string(),
                "Now with line numbers and a status bar.".to_string(),
                "Press 'q' to quit.".to_string(),
            ],
            status_message: String::new(),
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
        format!("Pretty Editor - {} lines", self.rows.len())
    }

    fn process_keypress(&mut self) {
        let mut buffer = [0; 1];
        if let Ok(_) = io::stdin().read_exact(&mut buffer) {
            match buffer[0] {
                b'q' => self.should_quit = true,
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
            print!("\x1b[2m{}\x1b[0m{}\r\n", line_number, row);
        }
    }

    fn draw_status_bar(&self) {
        let status = &self.status_message;
        let time = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

        print!(
            "\x1b[7m{:<width$}\x1b[0m\r\n",
            format!(" {} | Time: {}", status, time),
            width = 80
        );
    }

    fn clear
