use super::InstallationStage;
use crate::ui::{ProgressBar, ProgressStyle};
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct InitramfsStage;

impl InstallationStage for InitramfsStage {
    fn name(&self) -> &'static str {
        "Initial RAM Filesystem"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();

        let kernel_version = "5.4.0-42-generic";
        println!("{}", format!("update-initramfs: Generating /boot/initrd.img-{}", kernel_version).bright_white());
        thread::sleep(Duration::from_millis(800));

        let modules = [
            "kernel/drivers/ata/libata.ko",
            "kernel/drivers/scsi/scsi_mod.ko",
            "kernel/drivers/scsi/sd_mod.ko",
            "kernel/fs/ext4/ext4.ko",
            "kernel/fs/mbcache.ko",
            "kernel/fs/jbd2/jbd2.ko",
            "kernel/crypto/crc32c_generic.ko",
            "kernel/drivers/usb/host/xhci-hcd.ko",
            "kernel/drivers/usb/core/usbcore.ko",
        ];

        if exit_check() {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
        }

        for module in &modules {
            println!("{}", format!("Adding module: {}", module).dimmed());
            thread::sleep(Duration::from_millis(rng.gen_range(100..250)));
        }

        println!();
        let progress = ProgressBar::new(ProgressStyle::Block);
        progress.animate("Copying binaries and libraries:", rng.gen_range(2000..3500), exit_check)?;

        println!();
        println!("{}", "Creating initramfs image...".bright_white());
        thread::sleep(Duration::from_millis(rng.gen_range(1000..2000)));

        let size_mb = rng.gen_range(25..45);
        println!("{}", format!("Image size: {}MB", size_mb).bright_green());

        Ok(())
    }
}
