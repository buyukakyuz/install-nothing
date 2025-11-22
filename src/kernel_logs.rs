use rand::seq::SliceRandom;

/// Manages kernel log messages for authentic system output
pub struct KernelLogs {
    logs: Vec<String>,
}

impl KernelLogs {
    /// Load kernel logs from file or use defaults
    pub fn load() -> Self {
        let content = include_str!("../data/kernel.log");
        let logs = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(|line| Self::strip_timestamp(line))
            .collect();

        Self { logs }
    }

    /// Strip kernel timestamp from log line
    /// Converts "[    0.000000] message" to "message"
    fn strip_timestamp(line: &str) -> String {
        if let Some(start) = line.find('[') {
            if let Some(end) = line[start..].find(']') {
                let timestamp_end = start + end + 1;
                return line[timestamp_end..].trim_start().to_string();
            }
        }
        line.to_string()
    }

    /// Get all kernel logs
    pub fn all_logs(&self) -> &[String] {
        &self.logs
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
}

impl Default for KernelLogs {
    fn default() -> Self {
        Self::load()
    }
}
