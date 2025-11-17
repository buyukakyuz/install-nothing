mod bios;
mod boot;
mod bootloader;
mod compilation;
mod database;
mod drivers;
mod filesystem;
mod initramfs;
mod locale;
mod network;
mod optimization;
mod packages;
mod retro;
mod services;
mod system;
mod xorg;

use std::io;

pub use bios::BiosStage;
pub use boot::BootStage;
pub use bootloader::BootloaderStage;
pub use compilation::CompilationStage;
pub use database::DatabaseStage;
pub use drivers::DriversStage;
pub use filesystem::FilesystemStage;
pub use initramfs::InitramfsStage;
pub use locale::LocaleStage;
pub use network::NetworkStage;
pub use optimization::OptimizationStage;
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

/// Get all installation stages in order
pub fn all_stages() -> Vec<Box<dyn InstallationStage>> {
    vec![
        Box::new(BiosStage),
        Box::new(BootStage::new()),
        Box::new(BootloaderStage),
        Box::new(FilesystemStage),
        Box::new(SystemStage),
        Box::new(NetworkStage),
        Box::new(DriversStage),
        Box::new(InitramfsStage),
        Box::new(PackagesStage),
        Box::new(CompilationStage::new()),
        Box::new(DatabaseStage),
        Box::new(XorgStage),
        Box::new(ServicesStage),
        Box::new(RetroSoftwareStage),
        Box::new(LocaleStage),
        Box::new(OptimizationStage),
    ]
}
