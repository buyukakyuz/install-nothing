use super::InstallationStage;
use crate::messages::SYSTEM_COMPONENTS;
use crate::ui::{ProgressBar, ProgressStyle};
use colored::*;
use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub struct SystemStage;

impl InstallationStage for SystemStage {
    fn name(&self) -> &'static str {
        "System Component Installation"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();

        for component in SYSTEM_COMPONENTS {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            print!("  - {} ", component);
            io::stdout().flush()?;
            thread::sleep(Duration::from_millis(rng.gen_range(300..800)));
            println!("{}", "[OK]".bright_green());
        }

        println!();
        let progress = ProgressBar::new(ProgressStyle::Equals);
        progress.animate(
            "Building module dependencies:",
            rng.gen_range(2000..3500),
            exit_check,
        )?;

        Ok(())
    }
}
