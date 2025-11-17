use super::InstallationStage;
use crate::ui::{ProgressBar, ProgressStyle, Spinner};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct DatabaseStage;

impl InstallationStage for DatabaseStage {
    fn name(&self) -> &'static str {
        "Database Server Installation"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();
        let mut spinner = Spinner::new();

        let db_type = if rng.gen_bool(0.5) { "MySQL" } else { "PostgreSQL" };
        let version = if db_type == "MySQL" { "8.0.28" } else { "14.2" };

        println!("{}", format!("Installing {} Server {}...", db_type, version).bright_white());
        thread::sleep(Duration::from_millis(800));

        spinner.animate("Initializing database cluster...", 2000, exit_check)?;

        if db_type == "PostgreSQL" {
            println!("{}", "The files belonging to this database system will be owned by user \"postgres\".".dimmed());
            println!("{}", "This user must also own the server process.".dimmed());
            thread::sleep(Duration::from_millis(500));
        }

        println!();
        println!("{}", "Creating database files...".bright_white());

        let files = [
            "global/pg_control",
            "base/1/pg_filenode.map",
            "base/template1/pg_version",
            "pg_wal/000000010000000000000001",
            "pg_multixact/offsets/0000",
        ];

        for file in &files {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            println!("{}", format!("  creating {}", file).dimmed());
            thread::sleep(Duration::from_millis(rng.gen_range(150..300)));
        }

        println!();
        let progress = ProgressBar::new(ProgressStyle::Equals);
        progress.animate("Initializing system tables:", rng.gen_range(2000..3500), exit_check)?;

        println!();
        spinner.animate("Creating template databases...", 1500, exit_check)?;

        println!("{}", "Success. You can now start the database server using:".bright_green());
        println!("{}", format!("    {} -D /var/lib/{}/data",
            if db_type == "PostgreSQL" { "pg_ctl" } else { "mysqld" },
            db_type.to_lowercase()
        ).dimmed());

        Ok(())
    }
}
