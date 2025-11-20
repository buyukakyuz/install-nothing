mod ai;
mod bios;
mod boot;
mod bootloader;
mod cloud;
mod compilation;
mod container;
mod database;
mod deno;
mod drivers;
mod filesystem;
mod initramfs;
mod kernel;
mod locale;
mod network;
mod packages;
mod retro;
mod services;
mod system;
mod xorg;

use crate::cli::Stage;
use std::io;

pub use ai::AiStage;
pub use bios::BiosStage;
pub use boot::BootStage;
pub use bootloader::BootloaderStage;
pub use cloud::CloudStage;
pub use compilation::CompilationStage;
pub use container::ContainerStage;
pub use database::DatabaseStage;
pub use deno::DenoStage;
pub use drivers::DriversStage;
pub use filesystem::FilesystemStage;
pub use initramfs::InitramfsStage;
pub use kernel::KernelStage;
pub use locale::LocaleStage;
pub use network::NetworkStage;
pub use packages::PackagesStage;
pub use retro::RetroSoftwareStage;
pub use services::ServicesStage;
pub use system::SystemStage;
pub use xorg::XorgStage;

/// Common trait for all installation stages
pub trait InstallationStage {
    fn name(&self) -> &'static str;
    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()>;
}

use crate::config::SimulationConfig;

/// Get selected installation stages in order
pub fn selected_stages(stages: &[Stage]) -> Vec<Box<dyn InstallationStage>> {
    let mut result = Vec::new();
    let config = SimulationConfig::default();

    for stage in stages {
        let stage_impl: Box<dyn InstallationStage> = match stage {
            Stage::Bios => Box::new(BiosStage::new(config.bios.clone())),
            Stage::Boot => Box::new(BootStage::new(config.boot.clone())),
            Stage::Bootloader => Box::new(BootloaderStage::new(config.bootloader.clone())),
            Stage::Filesystem => Box::new(FilesystemStage),
            Stage::System => Box::new(SystemStage),
            Stage::Network => Box::new(NetworkStage),
            Stage::Drivers => Box::new(DriversStage),
            Stage::Initramfs => Box::new(InitramfsStage),
            Stage::Packages => Box::new(PackagesStage),
            Stage::Kernel => Box::new(KernelStage::new()),
            Stage::Compilation => Box::new(CompilationStage::new()),
            Stage::Deno => Box::new(DenoStage::new()),
            Stage::Database => Box::new(DatabaseStage),
            Stage::Xorg => Box::new(XorgStage),
            Stage::Services => Box::new(ServicesStage),
            Stage::Retro => Box::new(RetroSoftwareStage),
            Stage::Locale => Box::new(LocaleStage),
            Stage::Container => Box::new(ContainerStage::new(config.container.clone())),
            Stage::Ai => Box::new(AiStage::new(config.ai.clone())),
            Stage::Cloud => Box::new(CloudStage::new(config.cloud.clone())),
        };
        result.push(stage_impl);
    }

    result
}
