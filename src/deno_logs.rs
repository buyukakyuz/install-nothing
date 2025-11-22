/// Manages Deno compilation logs (both success and error cases)
pub struct DenoLogs {
    success_logs: Vec<String>,
    error_logs: Vec<String>,
}

impl DenoLogs {
    /// Load Deno logs from files
    pub fn load() -> Self {
        let success_content = include_str!("../data/deno.log");
        let error_content = include_str!("../data/error/deno.log");

        let success_logs = success_content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(String::from)
            .collect();

        let error_logs = error_content
            .lines()
            .filter(|line| !line.trim().is_empty())
            .map(String::from)
            .collect();

        Self {
            success_logs,
            error_logs,
        }
    }

    /// Get success logs
    pub fn success_logs(&self) -> &[String] {
        &self.success_logs
    }

    /// Get error logs
    pub fn error_logs(&self) -> &[String] {
        &self.error_logs
    }
}
