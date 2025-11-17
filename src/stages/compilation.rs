use super::InstallationStage;
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

        println!("{}", "make[1]: Entering directory '/usr/src/linux-headers-5.4.0'".dimmed());
        println!();

        let mut rng = rand::thread_rng();

        let log_count = rng.gen_range(15..30);
        let logs = self.build_logs.random_batch(log_count);

        for log in logs {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            println!("{}", log.cyan());
            thread::sleep(Duration::from_millis(rng.gen_range(80..250)));
        }

        println!();
        let progress = ProgressBar::new(ProgressStyle::Gradient);
        progress.animate(
            "Linking modules:",
            rng.gen_range(2000..4000),
            exit_check,
        )?;

        println!("{}", "make[1]: Leaving directory '/usr/src/linux-headers-5.4.0'".dimmed());

        Ok(())
    }
}

impl Default for CompilationStage {
    fn default() -> Self {
        Self::new()
    }
}
