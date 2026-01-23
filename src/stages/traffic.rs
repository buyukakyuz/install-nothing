use super::InstallationStage;
use crate::log_generator::LogGenerator;
use crate::ui::Spinner;
use colored::*;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::thread;
use std::time::Duration;

const LOGHUB_PATH: &str = "/Users/eoporto/loghub";

const LOG_SOURCES: &[(&str, &str)] = &[
    ("OpenSSH", "OpenSSH_2k.log"),
    ("HDFS", "HDFS_2k.log"),
    ("Apache", "Apache_2k.log"),
    ("Proxifier", "Proxifier_2k.log"),
    ("Linux", "Linux_2k.log"),
    ("Zookeeper", "Zookeeper_2k.log"),
];

pub struct TrafficStage {
    loghub_path: PathBuf,
}

impl TrafficStage {
    pub fn new() -> Self {
        Self {
            loghub_path: PathBuf::from(LOGHUB_PATH),
        }
    }

    fn load_logs(&self) -> Vec<(String, Vec<String>)> {
        let mut all_logs = Vec::new();

        for (source_name, filename) in LOG_SOURCES {
            let path = self.loghub_path.join(source_name).join(filename);
            if let Ok(content) = fs::read_to_string(&path) {
                let lines: Vec<String> = content
                    .lines()
                    .filter(|l| !l.trim().is_empty())
                    .map(|l| l.to_string())
                    .collect();
                if !lines.is_empty() {
                    all_logs.push((source_name.to_string(), lines));
                }
            }
        }

        all_logs
    }

    fn truncate_line(line: &str, max_width: usize) -> String {
        if line.len() > max_width {
            format!("{}...", &line[..max_width - 3])
        } else {
            line.to_string()
        }
    }
}

impl Default for TrafficStage {
    fn default() -> Self {
        Self::new()
    }
}

impl InstallationStage for TrafficStage {
    fn name(&self) -> &'static str {
        "Network Traffic Monitor"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!(
            "\n{}",
            format!("> {}", self.name()).bright_magenta().bold()
        );
        println!();

        let mut rng = rand::thread_rng();
        let mut spinner = Spinner::new();

        println!(
            "{} Initializing network traffic analyzer...",
            LogGenerator::timestamp().dimmed()
        );
        thread::sleep(Duration::from_millis(500));

        spinner.animate("Loading log samples from loghub...", 1500, exit_check)?;

        let logs = self.load_logs();

        if logs.is_empty() {
            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                "Warning: No log samples found in loghub directory".yellow()
            );
            return Ok(());
        }

        println!(
            "{} Loaded {} log sources: {}",
            LogGenerator::timestamp().dimmed(),
            logs.len().to_string().cyan(),
            logs.iter()
                .map(|(name, _)| name.as_str())
                .collect::<Vec<_>>()
                .join(", ")
                .dimmed()
        );
        thread::sleep(Duration::from_millis(300));

        println!(
            "{} Starting network traffic replay...",
            LogGenerator::timestamp().dimmed()
        );
        println!();

        let total_lines: usize = logs.iter().map(|(_, lines)| lines.len()).sum();
        let lines_to_show = rng.gen_range(80..150).min(total_lines);

        let mut shown = 0;
        let mut current_source_idx = rng.gen_range(0..logs.len());
        let mut line_indices: Vec<usize> = (0..logs[current_source_idx].1.len()).collect();
        line_indices.shuffle(&mut rng);
        let mut line_pos = 0;

        while shown < lines_to_show {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            if line_pos >= line_indices.len() || rng.gen_bool(0.15) {
                current_source_idx = rng.gen_range(0..logs.len());
                line_indices = (0..logs[current_source_idx].1.len()).collect();
                line_indices.shuffle(&mut rng);
                line_pos = 0;

                if rng.gen_bool(0.3) {
                    println!(
                        "{} {} {}",
                        LogGenerator::timestamp().dimmed(),
                        "--- switching to".dimmed(),
                        logs[current_source_idx].0.cyan()
                    );
                    thread::sleep(Duration::from_millis(200));
                }
            }

            let (source_name, lines) = &logs[current_source_idx];
            let line = &lines[line_indices[line_pos]];
            line_pos += 1;

            let prefix = format!("[{}]", source_name);
            let truncated = Self::truncate_line(line, 100);

            let colored_line = if line.contains("error") || line.contains("Error") || line.contains("ERROR") {
                truncated.red()
            } else if line.contains("warn") || line.contains("Warn") || line.contains("WARN") || line.contains("Failed") {
                truncated.yellow()
            } else if line.contains("Invalid") || line.contains("BREAK-IN") {
                truncated.bright_red()
            } else if line.contains("success") || line.contains("ok") || line.contains("Received block") {
                truncated.green()
            } else {
                truncated.dimmed()
            };

            println!(
                "{} {} {}",
                LogGenerator::timestamp().dimmed(),
                prefix.bright_blue(),
                colored_line
            );

            let delay = if rng.gen_bool(0.1) {
                rng.gen_range(5..20)
            } else {
                rng.gen_range(30..120)
            };
            thread::sleep(Duration::from_millis(delay));

            shown += 1;

            if shown % 30 == 0 && rng.gen_bool(0.4) {
                println!();
                println!(
                    "{} Processed {} network events...",
                    LogGenerator::timestamp().dimmed(),
                    shown.to_string().cyan()
                );
                thread::sleep(Duration::from_millis(300));
                println!();
            }
        }

        println!();
        println!(
            "{} Network traffic analysis complete.",
            LogGenerator::timestamp().dimmed()
        );
        println!(
            "{} Total events processed: {}",
            LogGenerator::timestamp().dimmed(),
            shown.to_string().bright_green()
        );

        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
}
