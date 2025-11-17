use super::InstallationStage;
use crate::messages::PACKAGES;
use crate::ui::{ProgressBar, ProgressStyle, Spinner};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct PackagesStage;

impl InstallationStage for PackagesStage {
    fn name(&self) -> &'static str {
        "Package Installation"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut spinner = Spinner::new();
        spinner.animate("Reading package lists...", 1200, exit_check)?;
        spinner.animate("Building dependency tree...", 1500, exit_check)?;

        let mut rng = rand::thread_rng();

        for package in PACKAGES {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            println!("Unpacking {}...", package.bright_white());

            let size_kb: u32 = rng.gen_range(512..8192);
            let speed_kb: u32 = rng.gen_range(64..512);

            let progress = ProgressBar::new(ProgressStyle::Hash);
            progress.animate(
                &format!("  ({:.1}MB @ {}KB/s)", size_kb as f32 / 1024.0, speed_kb),
                rng.gen_range(1000..2500),
                exit_check,
            )?;

            if rng.gen_bool(0.4) {
                println!("{}", format!("Setting up {}...", package).dimmed());
                thread::sleep(Duration::from_millis(300));
            }
        }

        println!("\n{}", "Processing triggers for shared libraries...".dimmed());
        thread::sleep(Duration::from_millis(800));
        println!("{}", "ldconfig: /usr/lib/x86_64-linux-gnu/libcrypto.so.1.1".dimmed());

        Ok(())
    }
}
