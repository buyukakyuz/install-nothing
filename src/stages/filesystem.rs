use super::InstallationStage;
use crate::log_generator::LogGenerator;
use crate::ui::{ProgressBar, ProgressStyle};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct FilesystemStage;

impl InstallationStage for FilesystemStage {
    fn name(&self) -> &'static str {
        "Filesystem Operations"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Creating ext4 filesystem on /dev/sda2...".bright_white()
        );
        thread::sleep(Duration::from_millis(600));

        let blocks = rng.gen_range(50000000..100000000);
        let inodes = blocks / 4;

        println!("{} {}", LogGenerator::timestamp().dimmed(), format!("mke2fs 1.45.5 (07-Jan-2020)").dimmed());
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            format!(
                "Creating filesystem with {} 4k blocks and {} inodes",
                blocks, inodes
            )
            .dimmed()
        );
        thread::sleep(Duration::from_millis(400));

        if exit_check() {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
        }

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Filesystem UUID: 8f3e1a2b-4c5d-6e7f-8a9b-0c1d2e3f4a5b".dimmed()
        );
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            format!("Superblock backups stored on blocks:").dimmed()
        );

        let backup_blocks = [32768, 98304, 163840, 229376, 294912];
        for block in &backup_blocks {
            println!("{} {}", LogGenerator::timestamp().dimmed(), format!("        {}", block).dimmed());
            thread::sleep(Duration::from_millis(100));
        }

        println!();
        let progress = ProgressBar::new(ProgressStyle::Equals);
        progress.animate(
            "Allocating group tables:",
            rng.gen_range(2000..3000),
            exit_check,
        )?;

        let progress = ProgressBar::new(ProgressStyle::Equals);
        progress.animate(
            "Writing inode tables:",
            rng.gen_range(2500..4000),
            exit_check,
        )?;

        println!("{} {}", LogGenerator::timestamp().dimmed(), "Creating journal (32768 blocks): ".dimmed());
        thread::sleep(Duration::from_millis(rng.gen_range(800..1200)));
        println!("{} {}", LogGenerator::timestamp().dimmed(), "done".bright_green());

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Writing superblocks and filesystem accounting information: ".dimmed()
        );
        thread::sleep(Duration::from_millis(rng.gen_range(600..1000)));
        println!("{} {}", LogGenerator::timestamp().dimmed(), "done".bright_green());

        println!();

        if rng.gen_bool(0.4) {
            println!("{} {}", LogGenerator::timestamp().dimmed(), "Running filesystem check...".bright_white());
            thread::sleep(Duration::from_millis(500));
            println!("{} {}", LogGenerator::timestamp().dimmed(), "e2fsck 1.45.5 (07-Jan-2020)".dimmed());
            println!("{} {}", LogGenerator::timestamp().dimmed(), "Pass 1: Checking inodes, blocks, and sizes".dimmed());
            thread::sleep(Duration::from_millis(rng.gen_range(800..1500)));
            println!("{} {}", LogGenerator::timestamp().dimmed(), "Pass 2: Checking directory structure".dimmed());
            thread::sleep(Duration::from_millis(rng.gen_range(600..1000)));
            println!("{} {}", LogGenerator::timestamp().dimmed(), "Pass 3: Checking directory connectivity".dimmed());
            thread::sleep(Duration::from_millis(rng.gen_range(400..800)));
            println!("{} {}", LogGenerator::timestamp().dimmed(), "Pass 4: Checking reference counts".dimmed());
            thread::sleep(Duration::from_millis(rng.gen_range(400..700)));
            println!("{} {}", LogGenerator::timestamp().dimmed(), "Pass 5: Checking group summary information".dimmed());
            thread::sleep(Duration::from_millis(rng.gen_range(300..600)));
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "/dev/sda2: 11/2048000 files (0.0% non-contiguous), 200000/8192000 blocks"
                    .bright_green()
            );
        }

        Ok(())
    }
}
