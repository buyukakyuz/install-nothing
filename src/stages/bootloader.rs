use super::InstallationStage;
use crate::config::BootloaderConfig;
use crate::log_generator::LogGenerator;
use crate::ui::{ProgressBar, ProgressStyle, Spinner};
use colored::*;
use rand::Rng;
use std::env;
use std::io;
use std::thread;
use std::time::Duration;

pub struct BootloaderStage {
    config: BootloaderConfig,
}

impl BootloaderStage {
    pub fn new(config: BootloaderConfig) -> Self {
        Self { config }
    }
}

impl InstallationStage for BootloaderStage {
    fn name(&self) -> &'static str {
        "Bootloader Installation"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!("\n{}", format!("> {}", self.name()).bright_yellow().bold());
        println!();

        let mut rng = rand::thread_rng();

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Installing GRUB2 bootloader...".bright_white()
        );
        thread::sleep(Duration::from_millis(self.config.install_delay));

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Running pre-installation checks...".dimmed()
        );
        thread::sleep(Duration::from_millis(500));

        let is_efi = rng.gen_bool(0.7);
        if is_efi {
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "EFI variables detected, installing for UEFI mode".dimmed()
            );
            thread::sleep(Duration::from_millis(300));
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "EFI System Partition found at /boot/efi".dimmed()
            );
        } else {
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "Legacy BIOS mode detected".dimmed()
            );
        }
        thread::sleep(Duration::from_millis(400));

        let mut spinner = Spinner::new();
        spinner.animate(
            "Probing devices for bootloader installation...",
            self.config.probe_delay,
            exit_check,
        )?;

        let devices = [
            ("/dev/sda", "SATA", "WDC WD10EZEX-08WN4A0", 1000),
            ("/dev/nvme0n1", "NVMe", "Samsung SSD 970 EVO Plus", 500),
            ("/dev/vda", "VirtIO", "QEMU HARDDISK", 64),
            ("/dev/sdb", "SATA", "Crucial MX500", 1000),
        ];
        let (device, dev_type, dev_name, capacity_gb) = devices[rng.gen_range(0..devices.len())];

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            format!("Detected {} device: {} ({})", dev_type, dev_name, device).dimmed()
        );
        thread::sleep(Duration::from_millis(250));
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            format!("  Capacity: {} GB", capacity_gb).dimmed()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            format!("  Block size: 512 bytes").dimmed()
        );
        thread::sleep(Duration::from_millis(300));

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Analyzing partition table...".dimmed()
        );
        thread::sleep(Duration::from_millis(600));

        let partition_table = if is_efi { "GPT" } else { "MBR" };
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            format!("  Partition table type: {}", partition_table).dimmed()
        );
        thread::sleep(Duration::from_millis(250));

        let boot_partition = if device == "/dev/nvme0n1" {
            format!("{}p2", device)
        } else {
            format!("{}2", device)
        };

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            format!("  Boot partition: {}", boot_partition).dimmed()
        );
        thread::sleep(Duration::from_millis(250));

        if is_efi {
            let efi_partition = if device == "/dev/nvme0n1" {
                format!("{}p1", device)
            } else {
                format!("{}1", device)
            };
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                format!("  EFI partition: {} (FAT32, 512 MB)", efi_partition).dimmed()
            );
            thread::sleep(Duration::from_millis(250));
        }
        thread::sleep(Duration::from_millis(400));

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Checking filesystems...".dimmed()
        );
        thread::sleep(Duration::from_millis(500));

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            format!("  {} is mounted on /", boot_partition).dimmed()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "  Filesystem: ext4".dimmed()
        );
        thread::sleep(Duration::from_millis(400));

        if exit_check() {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
        }

        let arch = env::consts::ARCH;
        let platform = match arch {
            "x86_64" => {
                if is_efi {
                    "x86_64-efi"
                } else {
                    "x86_64-pc"
                }
            }
            "aarch64" => {
                if is_efi {
                    "arm64-efi"
                } else {
                    "arm64-uboot"
                }
            }
            "x86" => "i386-pc",
            "arm" => "arm-uboot",
            _ => {
                if is_efi {
                    "efi"
                } else {
                    "pc"
                }
            }
        };

        println!(
            "\n{} {}",
            LogGenerator::timestamp().dimmed(),
            format!("Installing for {} platform...", platform).bright_white()
        );
        thread::sleep(Duration::from_millis(self.config.device_install_delay));

        let modules = if is_efi {
            vec![
                "part_gpt",
                "part_msdos",
                "fat",
                "ext2",
                "normal",
                "chain",
                "boot",
                "configfile",
                "linux",
                "efi_gop",
                "efi_uga",
                "font",
                "gfxterm",
                "gfxmenu",
                "all_video",
                "search",
                "search_fs_uuid",
            ]
        } else {
            vec![
                "part_msdos",
                "part_gpt",
                "ext2",
                "biosdisk",
                "normal",
                "chain",
                "boot",
                "configfile",
                "linux",
                "vbe",
                "vga",
                "font",
                "gfxterm",
                "gfxmenu",
                "all_video",
                "search",
            ]
        };

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            format!("Installing GRUB modules ({} modules)...", modules.len()).dimmed()
        );
        thread::sleep(Duration::from_millis(300));

        for (idx, module) in modules.iter().enumerate() {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            if idx % 3 == 0 {
                println!(
                    "{} {}",
                    LogGenerator::timestamp().dimmed(),
                    format!("  Installing {}.mod", module).dimmed()
                );
                thread::sleep(Duration::from_millis(rng.gen_range(150..300)));
            }
        }
        thread::sleep(Duration::from_millis(400));

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Installing bootloader fonts...".dimmed()
        );
        thread::sleep(Duration::from_millis(300));
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "  Converting DejaVu Sans Regular to PFF2 format".dimmed()
        );
        thread::sleep(Duration::from_millis(600));

        if exit_check() {
            return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
        }

        println!(
            "\n{} {}",
            LogGenerator::timestamp().dimmed(),
            "Generating grub configuration file...".bright_white()
        );
        thread::sleep(Duration::from_millis(self.config.config_gen_delay));

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Probing system configuration...".dimmed()
        );
        thread::sleep(Duration::from_millis(500));

        let kernel_sets = [
            [
                "vmlinuz-6.8.0-49-generic",
                "vmlinuz-6.8.0-48-generic",
                "vmlinuz-6.8.0-45-generic",
            ],
            [
                "vmlinuz-6.5.0-35-generic",
                "vmlinuz-6.5.0-28-generic",
                "vmlinuz-6.5.0-26-generic",
            ],
            [
                "vmlinuz-6.1.0-27-amd64",
                "vmlinuz-6.1.0-26-amd64",
                "vmlinuz-6.1.0-25-amd64",
            ],
        ];
        let kernels = &kernel_sets[rng.gen_range(0..kernel_sets.len())];

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Searching for linux images...".dimmed()
        );
        thread::sleep(Duration::from_millis(400));

        for kernel in kernels {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                format!("Found linux image: /boot/{}", kernel).dimmed()
            );

            let initrd_suffix = if kernel.contains("generic") {
                &kernel[8..]
            } else if kernel.contains("amd64") {
                &kernel[8..]
            } else {
                &kernel[8..]
            };

            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                format!("Found initrd image: /boot/initrd.img-{}", initrd_suffix).dimmed()
            );
            thread::sleep(Duration::from_millis(
                rng.gen_range(self.config.kernel_scan_delay_range.clone()),
            ));
        }

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Found memtest86+ image: /boot/memtest86+.bin".dimmed()
        );
        thread::sleep(Duration::from_millis(300));

        if rng.gen_bool(self.config.windows_found_chance) {
            let partition = if device == "/dev/nvme0n1" {
                format!("{}p3", device)
            } else {
                format!("{}3", device)
            };
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                format!("Found Windows Boot Manager on {}", partition).dimmed()
            );
            thread::sleep(Duration::from_millis(250));
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "  Windows 11 (loader) (on {})"
                    .replace("{}", &partition)
                    .dimmed()
            );
            thread::sleep(Duration::from_millis(200));
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "Adding boot menu entry for Windows".dimmed()
            );
            thread::sleep(Duration::from_millis(self.config.windows_delay));
        }

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Writing configuration to /boot/grub/grub.cfg...".dimmed()
        );
        thread::sleep(Duration::from_millis(500));

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "  Setting default boot entry: 0".dimmed()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "  Setting timeout: 5 seconds".dimmed()
        );
        thread::sleep(Duration::from_millis(200));
        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "  Enabling submenu for older kernels".dimmed()
        );
        thread::sleep(Duration::from_millis(500));

        if rng.gen_bool(0.6) {
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "Installing GRUB theme...".dimmed()
            );
            thread::sleep(Duration::from_millis(300));
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "  Theme: starfield".dimmed()
            );
            thread::sleep(Duration::from_millis(400));
        }

        println!(
            "\n{} {}",
            LogGenerator::timestamp().dimmed(),
            "Installing bootloader to disk...".bright_white()
        );
        thread::sleep(Duration::from_millis(300));

        if is_efi {
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "Creating EFI boot entry...".dimmed()
            );
            thread::sleep(Duration::from_millis(300));
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "  EFI application: \\EFI\\ubuntu\\shimx64.efi".dimmed()
            );
            thread::sleep(Duration::from_millis(400));
        }

        for i in 0..5 {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            let progress = ProgressBar::new(ProgressStyle::Block);
            progress.animate(
                &format!(
                    "{} Writing stage {} image...",
                    LogGenerator::timestamp().dimmed(),
                    i + 1
                ),
                rng.gen_range(self.config.write_stage_delay_range.clone()),
                exit_check,
            )?;
        }

        println!(
            "\n{} {}",
            LogGenerator::timestamp().dimmed(),
            "Verifying installation...".dimmed()
        );
        thread::sleep(Duration::from_millis(600));

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "  Checking boot sector... OK".dimmed()
        );
        thread::sleep(Duration::from_millis(400));

        if is_efi {
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "  Checking EFI boot variables... OK".dimmed()
            );
            thread::sleep(Duration::from_millis(400));
        }

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "  Verifying GRUB modules... OK".dimmed()
        );
        thread::sleep(Duration::from_millis(400));

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "  Checking configuration file... OK".dimmed()
        );
        thread::sleep(Duration::from_millis(400));

        println!(
            "{} {}",
            LogGenerator::timestamp().dimmed(),
            "Installation finished. No error reported.".bright_green()
        );

        thread::sleep(Duration::from_millis(self.config.finish_delay));

        Ok(())
    }
}
