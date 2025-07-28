use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
    ExecutableCommand,
};
use std::io::{self, stdout};

pub struct Terminal;

impl Terminal {
    pub fn default() -> Result<Self, std::io::Error> {
        terminal::enable_raw_mode()?;
        Ok(Self)
    }

    pub fn clear_screen() {
        let mut stdout = stdout();
        execute!(
            stdout,
            terminal::Clear(ClearType::All),
            cursor::MoveTo(0, 0)
        )
        .unwrap();
    }
}

impl Drop for Terminal {
    fn drop(&mut self) {
        terminal::disable_raw_mode().unwrap();
        stdout().execute(terminal::LeaveAlternateScreen).unwrap();
        stdout().execute(cursor::Show).unwrap();
    }
}
