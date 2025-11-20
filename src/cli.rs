use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, PartialEq, Eq, ValueEnum)]
pub enum Stage {
    /// BIOS initialization
    Bios,
    /// Boot sequence
    Boot,
    /// Bootloader installation
    Bootloader,
    /// Filesystem setup
    Filesystem,
    /// System installation
    System,
    /// Network configuration
    Network,
    /// Driver installation
    Drivers,
    /// Initramfs generation
    Initramfs,
    /// Package installation
    Packages,
    /// Linux kernel compilation
    Kernel,
    /// Compilation
    Compilation,
    /// Deno runtime compilation
    Deno,
    /// Database setup
    Database,
    /// X.org configuration
    Xorg,
    /// Services configuration
    Services,
    /// Retro software installation
    Retro,
    /// Locale configuration
    Locale,
    /// Container orchestration
    Container,
    /// AI model loading
    Ai,
    /// Cloud provisioning
    Cloud,
}

impl Stage {
    /// Returns all stages in installation order
    pub fn all() -> Vec<Stage> {
        vec![
            Stage::Bios,
            Stage::Boot,
            Stage::Bootloader,
            Stage::Filesystem,
            Stage::System,
            Stage::Network,
            Stage::Drivers,
            Stage::Initramfs,
            Stage::Packages,
            Stage::Kernel,
            Stage::Compilation,
            Stage::Deno,
            Stage::Database,
            Stage::Xorg,
            Stage::Services,
            Stage::Retro,
            Stage::Locale,
            Stage::Container,
            Stage::Ai,
            Stage::Cloud,
        ]
    }
}

#[derive(Parser, Debug)]
#[command(
    name = "install-nothing",
    version,
    about = "A nostalgic infinite installer simulator",
    long_about = "Universal System Installer - simulates an endless installation process.\n\
                  Select which stages to run, or use --all to run everything (default)."
)]
pub struct Cli {
    /// Stages to install (defaults to all if none specified)
    #[arg(value_enum)]
    pub stages: Vec<Stage>,

    /// Install all stages (default behavior)
    #[arg(short, long, conflicts_with = "stages")]
    pub all: bool,
}

impl Cli {
    /// Returns the selected stages, defaulting to all if none specified
    pub fn get_stages(&self) -> Vec<Stage> {
        if self.all || self.stages.is_empty() {
            Stage::all()
        } else {
            self.stages.clone()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_to_all() {
        let cli = Cli {
            stages: vec![],
            all: false,
        };
        assert_eq!(cli.get_stages(), Stage::all());
    }

    #[test]
    fn test_explicit_all() {
        let cli = Cli {
            stages: vec![],
            all: true,
        };
        assert_eq!(cli.get_stages(), Stage::all());
    }

    #[test]
    fn test_specific_stages() {
        let cli = Cli {
            stages: vec![Stage::Bios, Stage::Boot],
            all: false,
        };
        assert_eq!(cli.get_stages(), vec![Stage::Bios, Stage::Boot]);
    }
}
