use super::InstallationStage;
use colored::*;
use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub struct BootloaderStage;

impl InstallationStage for BootloaderStage {
    fn name(&self) -> &'static str {
        "Bootloader Installation"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();

        println!("{}", "Installing GRUB2 bootloader...".bright_white());
        thread::sleep(Duration::from_millis(800));

        println!("{}", "Probing devices for bootloader installation...".dimmed());
        thread::sleep(Duration::from_millis(600));

        let devices = ["/dev/sda", "/dev/nvme0n1", "/dev/vda"];
        let device = devices[rng.gen_range(0..devices.len())];
        println!("{}", format!("  Installing for x86_64-pc platform to {}", device).dimmed());
        thread::sleep(Duration::from_millis(500));

        if exit_check() {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
        }

        println!("\n{}", "Generating grub configuration file...".bright_white());
        thread::sleep(Duration::from_millis(700));

        let kernels = [
            "vmlinuz-5.4.0-42-generic",
            "vmlinuz-5.4.0-40-generic",
            "vmlinuz-5.4.0-39-generic",
        ];

        for kernel in &kernels {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            println!("{}", format!("Found linux image: /boot/{}", kernel).dimmed());
            println!("{}", format!("Found initrd image: /boot/initrd.img-{}", &kernel[8..]).dimmed());
            thread::sleep(Duration::from_millis(rng.gen_range(200..400)));
        }

        if rng.gen_bool(0.3) {
            println!("{}", "Found Windows Boot Manager on /dev/sda1".dimmed());
            thread::sleep(Duration::from_millis(400));
        }

        println!("\n{}", "Installing bootloader to disk...".bright_white());
        for i in 0..5 {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            print!("\r{}", format!("  Writing stage {} image...", i + 1).dimmed());
            io::stdout().flush()?;
            thread::sleep(Duration::from_millis(rng.gen_range(400..800)));
        }
        println!("\r{}", "  Installation finished. No error reported.".bright_green());

        thread::sleep(Duration::from_millis(500));

        Ok(())
    }
}
