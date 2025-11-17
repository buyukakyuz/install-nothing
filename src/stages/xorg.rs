use super::InstallationStage;
use crate::ui::Spinner;
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct XorgStage;

impl InstallationStage for XorgStage {
    fn name(&self) -> &'static str {
        "X Window System Setup"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();
        let mut spinner = Spinner::new();

        println!("{}", "Installing X.Org Server...".bright_white());
        thread::sleep(Duration::from_millis(600));

        let packages = [
            "xserver-xorg-core",
            "xserver-xorg-input-all",
            "xserver-xorg-video-intel",
            "xserver-xorg-video-nouveau",
            "libgl1-mesa-dri",
            "libglx-mesa0",
        ];

        for package in &packages {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            println!("{}", format!("  Setting up {}...", package).dimmed());
            thread::sleep(Duration::from_millis(rng.gen_range(200..500)));
        }

        println!();
        spinner.animate("Generating X server configuration...", 1500, exit_check)?;

        println!("{}", "Probing graphics hardware...".dimmed());
        thread::sleep(Duration::from_millis(800));

        let gpu_vendors = ["Intel", "NVIDIA", "AMD Radeon", "VirtualBox Graphics"];
        let gpu = gpu_vendors[rng.gen_range(0..gpu_vendors.len())];
        println!("{}", format!("  Detected: {} Graphics", gpu).bright_green());

        thread::sleep(Duration::from_millis(400));
        println!("{}", "  Resolution: 1920x1080@60Hz".dimmed());
        println!("{}", "  Color depth: 24-bit".dimmed());

        println!();
        spinner.animate("Building font cache...", 2000, exit_check)?;

        Ok(())
    }
}
