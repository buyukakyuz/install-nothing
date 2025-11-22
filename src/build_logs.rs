use rand::seq::SliceRandom;

/// Manages build log messages for authentic compilation output
pub struct BuildLogs {
    logs: Vec<String>,
}

impl BuildLogs {
    /// Load build logs from file or use defaults
    pub fn load() -> Self {
        let content = include_str!("../data/build.log");
        let logs = content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(String::from)
            .collect();

        Self { logs }
    }

    /// Get all build logs
    pub fn all_logs(&self) -> &[String] {
        &self.logs
    }

    #[allow(dead_code)]
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

impl Default for BuildLogs {
    fn default() -> Self {
        Self::load()
    }
}

