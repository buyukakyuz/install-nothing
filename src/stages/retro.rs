use super::InstallationStage;
use crate::messages::RETRO_SOFTWARE;
use crate::ui::{ProgressBar, ProgressStyle};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct RetroSoftwareStage;

impl InstallationStage for RetroSoftwareStage {
    fn name(&self) -> &'static str {
        "Retro Software Installation"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();

        for (name, version, size_kb) in RETRO_SOFTWARE {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            println!(
                "\n{} {} {}",
                "Installing".bright_white(),
                name.bright_cyan().bold(),
                format!("v{}", version).dimmed()
            );

            if rng.gen_bool(0.2) {
                println!("{}", "Checking for previous installation...".dimmed());
                thread::sleep(Duration::from_millis(500));
            }

            let progress = ProgressBar::new(ProgressStyle::Block);
            progress.animate(
                &format!("  Extracting files ({:.1}MB):", *size_kb as f32 / 1024.0),
                rng.gen_range(2000..4000),
                exit_check,
            )?;

            println!("{}", "  Creating shortcuts...".dimmed());
            thread::sleep(Duration::from_millis(400));

            if rng.gen_bool(0.3) {
                println!("{}", "  Registering file associations...".dimmed());
                thread::sleep(Duration::from_millis(300));
            }
        }

        Ok(())
    }
}
