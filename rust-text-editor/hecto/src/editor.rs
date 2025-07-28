use std::io::{self, Write};

use crate::terminal::Terminal;
use crossterm::event::{self, Event, KeyCode, KeyEvent};

pub struct Editor {
    should_quit: bool,
}

impl Default for Editor {
    fn default() -> Self {
        Self { should_quit: false }
    }
}

impl Editor {
    pub fn run(&mut self) {
        let _terminal = Terminal::default();

        loop {
            if let Err(error) = self.refresh_screen() {
                eprintln!("{error}");
                break;
            }

            if self.should_quit {
                break;
            }

            if let Err(error) = self.process_keypress() {
                eprintln!("{error}");
                break;
            }
        }
    }

    fn refresh_screen(&self) -> Result<(), std::io::Error> {
        Terminal::clear_screen();
        println!("~");
        io::stdout().flush()
    }

    fn process_keypress(&mut self) -> Result<(), std::io::Error> {
        if event::poll(std::time::Duration::from_millis(500))? {
            if let Event::Key(KeyEvent { code, .. }) = event::read()? {
                match code {
                    KeyCode::Char('q') => self.should_quit = true,
                    _ => {}
                }
            }
        }
        Ok(())
    }
}
