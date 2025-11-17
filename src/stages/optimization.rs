use super::InstallationStage;
use crate::messages::OPTIMIZATION_TASKS;
use crate::ui::{ProgressBar, ProgressStyle};
use colored::*;
use rand::Rng;
use std::io;

pub struct OptimizationStage;

impl InstallationStage for OptimizationStage {
    fn name(&self) -> &'static str {
        "System Optimization"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();

        for task in OPTIMIZATION_TASKS {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            let progress = ProgressBar::new(ProgressStyle::Equals);
            progress.animate(task, rng.gen_range(1500..3000), exit_check)?;
        }

        Ok(())
    }
}
