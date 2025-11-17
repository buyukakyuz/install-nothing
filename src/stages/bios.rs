use super::InstallationStage;
use crate::ui::{ProgressBar, ProgressStyle, Spinner};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct BiosStage;

impl InstallationStage for BiosStage {
    fn name(&self) -> &'static str {
        "BIOS/Firmware Update Sequence"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut spinner = Spinner::new();
        spinner.animate("Initializing BIOS ROM v2.3.1...", 1500, exit_check)?;
        spinner.animate("Detecting memory modules... 256MB RAM found", 1000, exit_check)?;

        let mut rng = rand::thread_rng();
        if rng.gen_bool(0.3) {
            println!("{}", "WARNING: CMOS checksum invalid, using defaults".yellow());
            thread::sleep(Duration::from_millis(800));
        }

        let progress = ProgressBar::new(ProgressStyle::Block);
        progress.animate(
            "Flashing firmware:",
            rng.gen_range(2000..4000),
            exit_check,
        )?;

        println!("{}", "  *** Do NOT power off during this process! ***".yellow());
        thread::sleep(Duration::from_millis(500));

        Ok(())
    }
}
