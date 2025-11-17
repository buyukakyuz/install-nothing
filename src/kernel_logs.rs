use rand::seq::SliceRandom;
use std::fs;
use std::path::Path;

/// Manages kernel log messages for authentic system output
pub struct KernelLogs {
    logs: Vec<String>,
}

impl KernelLogs {
    /// Load kernel logs from file or use defaults
    pub fn load() -> Self {
        let log_path = Path::new("data/kernel.log");

        let logs = if log_path.exists() {
            match fs::read_to_string(log_path) {
                Ok(content) => {
                    content
                        .lines()
                        .filter(|line| !line.trim().is_empty())
                        .map(|line| Self::strip_timestamp(line))
                        .collect()
                }
                Err(_) => Self::default_logs(),
            }
        } else {
            Self::default_logs()
        };

        Self { logs }
    }

    /// Strip kernel timestamp from log line
    /// Converts "[    0.000000] message" to "message"
    fn strip_timestamp(line: &str) -> String {
        // Check if line starts with '[' followed by timestamp pattern
        if let Some(start) = line.find('[') {
            if let Some(end) = line[start..].find(']') {
                let timestamp_end = start + end + 1;
                // Return everything after the closing bracket, trimmed
                return line[timestamp_end..].trim_start().to_string();
            }
        }
        // If no timestamp pattern found, return as is
        line.to_string()
    }

    /// Get multiple random kernel log messages
    pub fn random_batch(&self, count: usize) -> Vec<&str> {
        let mut rng = rand::thread_rng();
        let mut batch = Vec::new();

        for _ in 0..count {
            if let Some(log) = self.logs.choose(&mut rng) {
                batch.push(log.as_str());
            }
        }

        batch
    }

    /// Default fallback logs if no file is present
    fn default_logs() -> Vec<String> {
        vec![
            "[    0.000000] Linux version 5.4.0-42-generic (buildd@lgw01-amd64-060)".to_string(),
            "[    0.000000] Command line: BOOT_IMAGE=/boot/vmlinuz-5.4.0-42-generic".to_string(),
            "[    0.000000] KERNEL supported cpus:".to_string(),
            "[    0.000000] x86/fpu: Supporting XSAVE feature 0x001: 'x87 floating point registers'".to_string(),
            "[    0.000000] x86/fpu: Supporting XSAVE feature 0x002: 'SSE registers'".to_string(),
            "[    0.000000] x86/fpu: Enabled xstate features 0x7, context size is 832 bytes".to_string(),
            "[    0.000000] BIOS-provided physical RAM map:".to_string(),
            "[    0.000000] BIOS-e820: [mem 0x0000000000000000-0x000000000009fbff] usable".to_string(),
            "[    0.000000] NX (Execute Disable) protection: active".to_string(),
            "[    0.000000] SMBIOS 2.8 present.".to_string(),
            "[    0.000000] DMI: System manufacturer System Product Name/PRIME B350-PLUS".to_string(),
            "[    0.000000] e820: update [mem 0x00000000-0x00000fff] usable ==> reserved".to_string(),
            "[    0.000000] tsc: Fast TSC calibration using PIT".to_string(),
            "[    0.000000] Zone ranges:".to_string(),
            "[    0.000000]   DMA      [mem 0x0000000000001000-0x0000000000ffffff]".to_string(),
            "[    0.000000]   DMA32    [mem 0x0000000001000000-0x00000000ffffffff]".to_string(),
            "[    0.000000]   Normal   [mem 0x0000000100000000-0x000000041f5fffff]".to_string(),
            "[    0.000000] Movable zone start for each node".to_string(),
            "[    0.000000] Early memory node ranges".to_string(),
            "[    0.000000] Initmem setup node 0 [mem 0x0000000000001000-0x000000041f5fffff]".to_string(),
            "[    0.000000] ACPI: PM-Timer IO Port: 0x808".to_string(),
            "[    0.000000] ACPI: LAPIC_NMI (acpi_id[0xff] high edge lint[0x1])".to_string(),
            "[    0.000000] IOAPIC[0]: apic_id 9, version 33, address 0xfec00000, GSI 0-23".to_string(),
            "[    0.000000] ACPI: INT_SRC_OVR (bus 0 bus_irq 0 global_irq 2 dfl dfl)".to_string(),
            "[    0.000000] Using ACPI (MADT) for SMP configuration information".to_string(),
            "[    0.000000] smpboot: Allowing 8 CPUs, 0 hotplug CPUs".to_string(),
            "[    0.000000] PM: Registered nosave memory".to_string(),
            "[    0.000000] [mem 0xf0000000-0xffffffff] available for PCI devices".to_string(),
            "[    0.000000] clocksource: refined-jiffies: mask: 0xffffffff max_cycles".to_string(),
            "[    0.000000] setup_percpu: NR_CPUS:8192 nr_cpumask_bits:8 nr_cpu_ids:8 nr_node_ids:1".to_string(),
        ]
    }
}

impl Default for KernelLogs {
    fn default() -> Self {
        Self::load()
    }
}
