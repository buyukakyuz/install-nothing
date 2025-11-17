mod build_logs;
mod installer;
mod kernel_logs;
mod messages;
mod stages;
mod ui;

use colored::*;
use installer::Installer;
use std::io;

fn main() {
    if let Err(e) = run_installer() {
        handle_error(e);
    }
}

fn run_installer() -> io::Result<()> {
    let mut installer = Installer::new();
    installer.run()
}

fn handle_error(e: io::Error) {
    if e.kind() == io::ErrorKind::Interrupted {
        println!(
            "\n\n{}",
            "═══════════════════════════════════════".bright_cyan()
        );
        println!("{}", "Installation cancelled by user.".bright_white());
        println!(
            "{}",
            "Thank you for using Universal System Installer!".bright_white()
        );
        println!(
            "{}",
            "═══════════════════════════════════════".bright_cyan()
        );
    } else {
        eprintln!("\n{} {:?}", "Error:".bright_red(), e);
        std::process::exit(1);
    }
}
