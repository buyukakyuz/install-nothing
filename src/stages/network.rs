use super::InstallationStage;
use crate::ui::Spinner;
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct NetworkStage;

impl InstallationStage for NetworkStage {
    fn name(&self) -> &'static str {
        "Network Configuration"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();
        let mut spinner = Spinner::new();

        println!("{}", "Configuring network interfaces...".bright_white());
        thread::sleep(Duration::from_millis(500));

        let interfaces = ["eth0", "enp0s3", "wlan0"];
        let interface = interfaces[rng.gen_range(0..interfaces.len())];

        println!("{}", format!("  Interface: {}", interface).dimmed());
        thread::sleep(Duration::from_millis(300));

        if exit_check() {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
        }

        spinner.animate(&format!("Requesting DHCP lease on {}...", interface), 2000, exit_check)?;

        let ip = format!("192.168.{}.{}", rng.gen_range(0..255), rng.gen_range(2..254));
        let gateway = format!("192.168.{}.1", rng.gen_range(0..255));

        println!("{}", format!("  IP Address: {}", ip).bright_green());
        println!("{}", format!("  Netmask: 255.255.255.0").dimmed());
        println!("{}", format!("  Gateway: {}", gateway).dimmed());
        println!("{}", format!("  DNS: 8.8.8.8, 8.8.4.4").dimmed());
        thread::sleep(Duration::from_millis(600));

        println!();
        spinner.animate("Configuring DNS resolution...", 1200, exit_check)?;

        println!("{}", "Updating /etc/resolv.conf".dimmed());
        thread::sleep(Duration::from_millis(400));

        if rng.gen_bool(0.3) {
            println!();
            spinner.animate("Testing network connectivity...", 1500, exit_check)?;
            println!("{}", "Network is reachable".bright_green());
        }

        Ok(())
    }
}
