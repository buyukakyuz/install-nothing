use super::InstallationStage;
use crate::ui::{ProgressBar, ProgressStyle};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct LocaleStage;

impl InstallationStage for LocaleStage {
    fn name(&self) -> &'static str {
        "Localization Configuration"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();

        println!("{}", "Generating locales...".bright_white());
        thread::sleep(Duration::from_millis(500));

        let locales = [
            "en_US.UTF-8",
            "en_GB.UTF-8",
            "de_DE.UTF-8",
            "fr_FR.UTF-8",
            "es_ES.UTF-8",
            "ja_JP.UTF-8",
            "zh_CN.UTF-8",
        ];

        for locale in &locales {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            println!("{}", format!("Generating locale {}... ", locale).dimmed());
            thread::sleep(Duration::from_millis(rng.gen_range(300..700)));
            println!("{}", "done".bright_green());
        }

        println!();
        let progress = ProgressBar::new(ProgressStyle::Hash);
        progress.animate("Building locale archive:", rng.gen_range(2000..3000), exit_check)?;

        println!();
        println!("{}", "Configuring timezone...".bright_white());
        let timezones = [
            "America/New_York",
            "America/Los_Angeles",
            "Europe/London",
            "Europe/Berlin",
            "Asia/Tokyo",
        ];
        let timezone = timezones[rng.gen_range(0..timezones.len())];
        println!("{}", format!("  Timezone set to: {}", timezone).dimmed());
        thread::sleep(Duration::from_millis(600));

        Ok(())
    }
}
