use colored::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub struct Spinner {
    chars: Vec<char>,
    index: usize,
}

impl Spinner {
    pub fn new() -> Self {
        Self {
            chars: vec!['|', '/', '-', '\\'],
            index: 0,
        }
    }

    pub fn next(&mut self) -> char {
        let c = self.chars[self.index];
        self.index = (self.index + 1) % self.chars.len();
        c
    }

    pub fn animate(
        &mut self,
        message: &str,
        duration_ms: u64,
        exit_check: &dyn Fn() -> bool,
    ) -> io::Result<()> {
        let steps = duration_ms / 100;
        for _ in 0..steps {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            print!("\r{} {}", self.next(), message.bright_white());
            io::stdout().flush()?;
            thread::sleep(Duration::from_millis(100));
        }
        print!("\r{} {}\n", "✓".bright_green(), message.bright_white());
        Ok(())
    }
}

impl Default for Spinner {
    fn default() -> Self {
        Self::new()
    }
}
