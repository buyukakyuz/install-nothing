use super::InstallationStage;
use colored::*;
use rand::Rng;
use std::io::{self, Write};
use std::thread;
use std::time::Duration;

pub struct ServicesStage;

impl InstallationStage for ServicesStage {
    fn name(&self) -> &'static str {
        "System Services Configuration"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();

        let services = [
            ("NetworkManager.service", "Network Manager"),
            ("systemd-resolved.service", "Network Name Resolution"),
            ("ssh.service", "OpenSSH server daemon"),
            ("cron.service", "Regular background program processing daemon"),
            ("rsyslog.service", "System Logging Service"),
            ("dbus.service", "D-Bus System Message Bus"),
            ("avahi-daemon.service", "Avahi mDNS/DNS-SD Stack"),
            ("cups.service", "CUPS Scheduler"),
            ("bluetooth.service", "Bluetooth service"),
            ("apache2.service", "The Apache HTTP Server"),
        ];

        println!("{}", "Starting system services...".bright_white());
        println!();

        for (_service, description) in &services {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            print!("{}", format!("[ ** ] Starting {}...", description).bright_cyan());
            io::stdout().flush()?;
            thread::sleep(Duration::from_millis(rng.gen_range(300..800)));
            print!("\r");
            println!("{}", format!("[ OK ] Started {}.", description).bright_green());
            thread::sleep(Duration::from_millis(rng.gen_range(100..300)));
        }

        println!();
        println!("{}", format!("Loaded {} services, {} active", services.len(), services.len()).dimmed());

        Ok(())
    }
}
