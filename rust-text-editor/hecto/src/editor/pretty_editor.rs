use std::io::{self, Read, Write};
use std::time::{SystemTime, UNIX_EPOCH};

// Colors
const CYAN: &str = "\x1b[36m";
const BLUE_BG: &str = "\x1b[44m";
const YELLOW_BG: &str = "\x1b[43m";
const DIM: &str = "\x1b[2m";
const RESET: &str = "\x1b[0m";
const CLEAR: &str = "\x1b[2J\x1b[H";

// Config
const WIDTH: usize = 80;
const PADDING: usize = 4;

pub struct PrettyEditor {
    rows: [&'static str; 6],
    cursor: usize,
}

impl PrettyEditor {
    pub fn new() -> Self {
        Self {
            rows: [
                "🌈 Welcome to the Enhanced Editor!",
                "Use 'j'/'k' to move, 'q' to quit",
                "Line numbers are cyan 💙",
                "Current line is blue-highlighted 💡",
                "Status bar is dynamic and clean.",
