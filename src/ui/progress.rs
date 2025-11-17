use colored::*;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

#[derive(Clone, Copy)]
pub enum ProgressStyle {
    Hash,      // [####......]
    Equals,    // [========..]
    Block,     // [████░░░░░░]
    Gradient,  // [▓▓▓▓▒▒▒░░░]
}

pub struct ProgressBar {
    width: usize,
    style: ProgressStyle,
}

impl ProgressBar {
    pub fn new(style: ProgressStyle) -> Self {
        Self { width: 20, style }
    }

    pub fn render(&self, progress: f32) -> String {
        let filled = ((progress * self.width as f32) as usize).min(self.width);
        let empty = self.width - filled;

        let (fill_char, empty_char) = match self.style {
            ProgressStyle::Hash => ('#', '.'),
            ProgressStyle::Equals => ('=', ' '),
            ProgressStyle::Block => ('█', '░'),
            ProgressStyle::Gradient => {
                if filled > empty {
                    ('▓', '░')
                } else {
                    ('▒', '░')
                }
            }
        };

        format!(
            "[{}{}] {:3.0}%",
            fill_char.to_string().repeat(filled).bright_green(),
            empty_char.to_string().repeat(empty).dimmed(),
            progress * 100.0
        )
    }

    pub fn animate(
        &self,
        message: &str,
        duration_ms: u64,
        exit_check: &dyn Fn() -> bool,
    ) -> io::Result<()> {
        let steps = 50;
        let delay = duration_ms / steps;
        print!("{}", message.bright_white());
        io::stdout().flush()?;

        for i in 0..=steps {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            let progress = i as f32 / steps as f32;
            print!("\r{} {}", message.bright_white(), self.render(progress));
            io::stdout().flush()?;
            thread::sleep(Duration::from_millis(delay));
        }
        println!();
        Ok(())
    }
}
