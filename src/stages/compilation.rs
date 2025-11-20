use super::InstallationStage;
use crate::log_generator::LogGenerator;
use crate::build_logs::BuildLogs;
use crate::ui::{ProgressBar, ProgressStyle};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct CompilationStage {
    build_logs: BuildLogs,
}

impl CompilationStage {
    pub fn new() -> Self {
        Self {
            build_logs: BuildLogs::load(),
        }
    }
}

impl InstallationStage for CompilationStage {
    fn name(&self) -> &'static str {
        "Kernel Module Compilation"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "make[1]: Entering directory '/usr/src/linux-headers-5.4.0'".dimmed()
        );
        println!();

        let mut rng = rand::thread_rng();
        let logs = self.build_logs.all_logs();

        for log in logs {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            let should_show_progress = log.trim().starts_with("CC")
                || log.trim().starts_with("LD")
                || log.trim().starts_with("AR");

            if should_show_progress {
                let speed_category = rng.gen_range(0..10);
                let duration = if speed_category < 3 {
                    rng.gen_range(50..200)
                } else if speed_category < 7 {
                    rng.gen_range(200..600)
                } else {
                    rng.gen_range(600..1500)
                };

                let progress = ProgressBar::new(ProgressStyle::Block);
                progress.animate(&format!("{} {}", LogGenerator::timestamp().dimmed(), log.cyan()), duration, exit_check)?;
            } else {
                println!("{} {}", LogGenerator::timestamp().dimmed(), log.cyan());
                let speed_category = rng.gen_range(0..10);
                let delay = if speed_category < 4 {
                    rng.gen_range(10..30)
                } else if speed_category < 8 {
                    rng.gen_range(30..80)
                } else {
                    rng.gen_range(80..200)
                };
                thread::sleep(Duration::from_millis(delay));
            }
        }

        println!();
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "make[1]: Leaving directory '/usr/src/linux-headers-5.4.0'".dimmed()
        );

        Ok(())
    }
}

impl Default for CompilationStage {
    fn default() -> Self {
        Self::new()
    }
}
