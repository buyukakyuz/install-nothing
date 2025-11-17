use super::InstallationStage;
use crate::messages::DRIVERS;
use colored::*;
use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub struct DriversStage;

impl InstallationStage for DriversStage {
    fn name(&self) -> &'static str {
        "Hardware Driver Installation"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();

        for (device, driver) in DRIVERS {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            print!("Detecting: {} ", device.bright_cyan());
            io::stdout().flush()?;
            thread::sleep(Duration::from_millis(rng.gen_range(400..900)));

            println!("{}", "[FOUND]".bright_green());
            println!("  └─ Loading driver: {}", driver.dimmed());
            thread::sleep(Duration::from_millis(300));
        }

        println!();
        Ok(())
    }
}
