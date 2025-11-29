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

    /// Exclude specific stages from installation
    #[arg(short, long, value_enum, num_args = 0.., conflicts_with = "stages")]
    pub exclude: Vec<Stage>,
}

impl Cli {
    /// Returns the selected stages, defaulting to all if none specified
    pub fn get_stages(&self) -> Vec<Stage> {
        let mut stages = if self.all || self.stages.is_empty() {
            Stage::all()
        } else {
            self.stages.clone()
        };

        if !self.exclude.is_empty() {
            stages.retain(|stage| !self.exclude.contains(stage));
        }

        stages
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
            exclude: vec![],
        };
        assert_eq!(cli.get_stages(), Stage::all());
    }

    #[test]
    fn test_explicit_all() {
        let cli = Cli {
            stages: vec![],
            all: true,
            exclude: vec![],
        };
        assert_eq!(cli.get_stages(), Stage::all());
    }

    #[test]
    fn test_specific_stages() {
        let cli = Cli {
            stages: vec![Stage::Bios, Stage::Boot],
            all: false,
            exclude: vec![],
        };
        assert_eq!(cli.get_stages(), vec![Stage::Bios, Stage::Boot]);
    }

    #[test]
    fn test_exclude_single_stage() {
        let cli = Cli {
            stages: vec![],
            all: false,
            exclude: vec![Stage::Ai],
        };
        let result = cli.get_stages();
        assert!(!result.contains(&Stage::Ai));
        assert_eq!(result.len(), Stage::all().len() - 1);
    }

    #[test]
    fn test_exclude_multiple_stages() {
        let cli = Cli {
            stages: vec![],
            all: true,
            exclude: vec![Stage::Ai, Stage::Cloud],
        };
        let result = cli.get_stages();
        assert!(!result.contains(&Stage::Ai));
        assert!(!result.contains(&Stage::Cloud));
        assert_eq!(result.len(), Stage::all().len() - 2);
    }

    #[test]
    fn test_exclude_all_stages() {
        let cli = Cli {
            stages: vec![],
            all: false,
            exclude: Stage::all(),
        };
        let result = cli.get_stages();
        assert_eq!(result.len(), 0);
    }
}
