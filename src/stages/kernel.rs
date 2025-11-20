use super::InstallationStage;
use crate::log_generator::LogGenerator;
use crate::kernel_logs::KernelLogs;
use crate::ui::{ProgressBar, ProgressStyle};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct KernelStage {
    kernel_logs: KernelLogs,
}

impl KernelStage {
    pub fn new() -> Self {
        Self {
            kernel_logs: KernelLogs::load(),
        }
    }

    /// Display all kernel logs with progress bars for initialization steps
    fn display_logs(&self, logs: &[String], exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        let mut rng = rand::thread_rng();

        for log in logs {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            let should_show_progress = log.contains("Initializing")
                || log.contains("Loading")
                || log.contains("Mounting")
                || log.contains("Starting")
                || log.contains("Probing");

            if should_show_progress {
                let speed_category = rng.gen_range(0..10);
                let duration = if speed_category < 3 {
                    rng.gen_range(600..900)
                } else if speed_category < 7 {
                    rng.gen_range(900..1200)
                } else {
                    rng.gen_range(1200..3500)
                };

                let progress = ProgressBar::new(ProgressStyle::Block);
                progress.animate(&format!("{} {}", LogGenerator::timestamp().dimmed(), log.bright_cyan()), duration, exit_check)?;
            } else {
                let speed_category = rng.gen_range(0..10);
                let delay = if speed_category < 4 {
                    rng.gen_range(25..50)
                } else if speed_category < 8 {
                    rng.gen_range(50..100)
                } else {
                    rng.gen_range(100..400)
                };

                println!("{} {}", LogGenerator::timestamp().dimmed(), log.dimmed());
                thread::sleep(Duration::from_millis(delay));
            }
        }

        Ok(())
    }
}

impl InstallationStage for KernelStage {
    fn name(&self) -> &'static str {
        "Linux Kernel Compilation"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        println!("{} {}", LogGenerator::timestamp().dimmed(), "Building Linux kernel from source...".bright_white());
        println!();

        self.display_logs(self.kernel_logs.all_logs(), exit_check)?;

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Kernel build completed successfully!".bright_green().bold()
        );

        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
}

impl Default for KernelStage {
    fn default() -> Self {
        Self::new()
    }
}
