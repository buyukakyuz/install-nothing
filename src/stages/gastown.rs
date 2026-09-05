use super::InstallationStage;
use crate::log_generator::LogGenerator;
use crate::ui::Spinner;
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct GastownStage;

impl GastownStage {
    pub fn new() -> Self {
        Self
    }
}

impl Default for GastownStage {
    fn default() -> Self {
        Self::new()
    }
}

impl InstallationStage for GastownStage {
    fn name(&self) -> &'static str {
        "Gas Town Orchestration"
    }

    fn run(&self, exit_check: &dyn Fn() -> bool) -> io::Result<()> {
        println!(
            "\n{}",
            format!("> {}", self.name()).bright_cyan().bold()
        );
        println!();

        let mut rng = rand::thread_rng();
        let mut spinner = Spinner::new();

        println!(
            "{} {} Initializing Gas Town workspace...",
            LogGenerator::timestamp().dimmed(),
            "→".bright_blue()
        );
        thread::sleep(Duration::from_millis(500));

        spinner.animate("Loading agent configurations...", 1000, exit_check)?;

        let roles = ["mayor", "polecat", "crew", "witness", "deacon", "dog", "refinery"];
        println!(
            "{} {} Loaded {} agent roles: {}",
            LogGenerator::timestamp().dimmed(),
            "✓".green(),
            roles.len().to_string().cyan(),
            roles.join(", ").dimmed()
        );
        thread::sleep(Duration::from_millis(300));

        spinner.animate("Scanning rigs and convoys...", 800, exit_check)?;

        let rigs = ["api-service", "frontend", "infra", "docs", "ml-pipeline"];
        let active_rig = rigs[rng.gen_range(0..rigs.len())];
        println!(
            "{} {} Found {} rigs, active: {}",
            LogGenerator::timestamp().dimmed(),
            "✓".green(),
            rigs.len().to_string().cyan(),
            active_rig.bright_yellow()
        );
        thread::sleep(Duration::from_millis(400));

        println!(
            "{} {} Starting agent orchestration...",
            LogGenerator::timestamp().dimmed(),
            "→".bright_blue()
        );
        println!();

        let lines_to_show = rng.gen_range(60..100);
        let mut shown = 0;

        let mut state = GastownState::new(&mut rng);

        while shown < lines_to_show {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            let (log_line, level) = state.generate_event(&mut rng);
            let colored_line = match level {
                LogLevel::Success => log_line.green(),
                LogLevel::Warning => log_line.yellow(),
                LogLevel::Error => log_line.red(),
                LogLevel::Info => log_line.dimmed(),
                LogLevel::Action => log_line.bright_blue(),
                LogLevel::Agent => log_line.bright_magenta(),
            };

            println!(
                "{} {}",
                LogGenerator::timestamp().dimmed(),
                colored_line
            );

            let delay = if rng.gen_bool(0.15) {
                rng.gen_range(100..400)
            } else {
                rng.gen_range(40..150)
            };
            thread::sleep(Duration::from_millis(delay));

            shown += 1;

            if shown % 25 == 0 && rng.gen_bool(0.5) {
                println!();
                println!(
                    "{} {} Patrol cycle {} complete",
                    LogGenerator::timestamp().dimmed(),
                    "○".cyan(),
                    (shown / 25).to_string().cyan()
                );
                thread::sleep(Duration::from_millis(300));
                println!();
            }
        }

        println!();
        println!(
            "{} {} Gas Town orchestration cycle complete",
            LogGenerator::timestamp().dimmed(),
            "✓".green()
        );
        println!(
            "{} {} {} events processed, {} agents active",
            LogGenerator::timestamp().dimmed(),
            "→".bright_blue(),
            shown.to_string().bright_green(),
            state.active_polecats.len().to_string().cyan()
        );

        thread::sleep(Duration::from_millis(500));
        Ok(())
    }
}

#[derive(Clone, Copy)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Success,
    Action,
    Agent,
}

struct GastownState {
    active_polecats: Vec<String>,
    active_convoys: Vec<String>,
    current_rig: String,
}

impl GastownState {
    fn new(rng: &mut rand::rngs::ThreadRng) -> Self {
        let rigs = ["api-service", "frontend", "infra", "docs", "ml-pipeline"];
        Self {
            active_polecats: Vec::new(),
            active_convoys: Vec::new(),
            current_rig: rigs[rng.gen_range(0..rigs.len())].to_string(),
        }
    }

    fn generate_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        match rng.gen_range(0..20) {
            0..=2 => self.spawn_event(rng),
            3..=4 => self.wake_event(rng),
            5..=7 => self.nudge_event(rng),
            8 => self.handoff_event(rng),
            9..=10 => self.done_event(rng),
            11 => self.crash_event(rng),
            12 => self.kill_event(rng),
            13..=14 => self.patrol_event(rng),
            15..=16 => self.convoy_event(rng),
            17 => self.health_event(rng),
            18 => self.hook_event(rng),
            _ => self.mail_event(rng),
        }
    }

    fn spawn_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        let polecat_names = ["Toast", "Maple", "Rocket", "Nimbus", "Echo", "Pixel", "Cipher", "Nova", "Drift", "Spark"];
        let name = polecat_names[rng.gen_range(0..polecat_names.len())];
        let issue_id = random_issue_id(rng);

        if !self.active_polecats.contains(&name.to_string()) {
            self.active_polecats.push(name.to_string());
        }

        (
            format!("[spawn] {}/polecats/{} spawned for {}", self.current_rig, name, issue_id),
            LogLevel::Agent,
        )
    }

    fn wake_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        let roles = ["mayor", "witness", "deacon", "crew/max", "crew/dev"];
        let role = roles[rng.gen_range(0..roles.len())];
        let reasons = ["scheduled", "mail received", "health check", "nudge pending"];
        let reason = reasons[rng.gen_range(0..reasons.len())];

        (
            format!("[wake] {} resumed ({})", role, reason),
            LogLevel::Info,
        )
    }

    fn nudge_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        let targets: Vec<String> = if !self.active_polecats.is_empty() && rng.gen_bool(0.6) {
            vec![format!("{}/polecats/{}", self.current_rig, self.active_polecats[rng.gen_range(0..self.active_polecats.len())])]
        } else {
            vec!["mayor".to_string(), "witness".to_string(), "deacon".to_string()]
        };
        let target = &targets[rng.gen_range(0..targets.len())];

        let messages = [
            "Check your hook for work assignments",
            "Run 'gt prime' to check worker status",
            "Process pending callbacks",
            "Review convoy status",
            "Sync beads and check mail",
        ];
        let msg = messages[rng.gen_range(0..messages.len())];

        (
            format!("[nudge] {} nudged with \"{}\"", target, msg),
            LogLevel::Action,
        )
    }

    fn handoff_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        if self.active_polecats.is_empty() {
            return self.spawn_event(rng);
        }
        let polecat = &self.active_polecats[rng.gen_range(0..self.active_polecats.len())];
        let issue_id = random_issue_id(rng);

        (
            format!("[handoff] {}/polecats/{} handed off {} to fresh session", self.current_rig, polecat, issue_id),
            LogLevel::Info,
        )
    }

    fn done_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        if self.active_polecats.is_empty() {
            return self.spawn_event(rng);
        }
        let idx = rng.gen_range(0..self.active_polecats.len());
        let polecat = self.active_polecats.remove(idx);
        let issue_id = random_issue_id(rng);

        (
            format!("[done] {}/polecats/{} completed {}", self.current_rig, polecat, issue_id),
            LogLevel::Success,
        )
    }

    fn crash_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        let agents = ["polecat/Cipher", "witness", "deacon", "polecat/Nova"];
        let agent = agents[rng.gen_range(0..agents.len())];
        let reasons = [
            "exit code 137 (OOMKilled)",
            "exit code 1 (error)",
            "session terminated unexpectedly",
            "tmux pane closed",
        ];
        let reason = reasons[rng.gen_range(0..reasons.len())];

        (
            format!("[crash] {} exited: {}", agent, reason),
            LogLevel::Error,
        )
    }

    fn kill_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        if self.active_polecats.is_empty() {
            return self.health_event(rng);
        }
        let idx = rng.gen_range(0..self.active_polecats.len());
        let polecat = self.active_polecats.remove(idx);
        let reasons = ["stuck threshold exceeded", "health check failed", "manual termination"];
        let reason = reasons[rng.gen_range(0..reasons.len())];

        (
            format!("[kill] {}/polecats/{} killed: {}", self.current_rig, polecat, reason),
            LogLevel::Warning,
        )
    }

    fn patrol_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        let events = [
            ("patrol_started", "witness beginning patrol cycle", LogLevel::Info),
            ("polecat_checked", &format!("{} polecats responding", self.active_polecats.len()), LogLevel::Info),
            ("polecat_nudged", "sent reminder to idle worker", LogLevel::Action),
            ("escalation_sent", "escalated stuck worker to mayor", LogLevel::Warning),
            ("patrol_complete", "patrol cycle finished", LogLevel::Success),
        ];
        let (event_type, detail, level) = events[rng.gen_range(0..events.len())];

        (
            format!("[{}] {}", event_type, detail),
            level,
        )
    }

    fn convoy_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        let convoy_id = format!("convoy-{}", rng.gen_range(100..999));

        match rng.gen_range(0..5) {
            0 => {
                self.active_convoys.push(convoy_id.clone());
                let count = rng.gen_range(3..12);
                (
                    format!("✓ Created {} with {} issues", convoy_id, count),
                    LogLevel::Success,
                )
            }
            1 => {
                let issue_id = random_issue_id(rng);
                (
                    format!("→ Added {} to active convoy", issue_id),
                    LogLevel::Action,
                )
            }
            2 => {
                if !self.active_convoys.is_empty() {
                    let idx = rng.gen_range(0..self.active_convoys.len());
                    let c = self.active_convoys.remove(idx);
                    (
                        format!("✓ Closed {} (all issues resolved)", c),
                        LogLevel::Success,
                    )
                } else {
                    (
                        format!("→ Scanning for ready issues..."),
                        LogLevel::Info,
                    )
                }
            }
            3 => {
                let pct = rng.gen_range(25..95);
                (
                    format!("○ Convoy progress: {}% complete", pct),
                    LogLevel::Info,
                )
            }
            _ => {
                (
                    format!("⚠ Warning: couldn't track {}: issue not found", random_issue_id(rng)),
                    LogLevel::Warning,
                )
            }
        }
    }

    fn health_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        let agents = ["mayor", "witness", "deacon", "polecat/Toast", "polecat/Maple"];
        let agent = agents[rng.gen_range(0..agents.len())];

        match rng.gen_range(0..4) {
            0 => (
                format!("✓ {} health check passed", agent),
                LogLevel::Success,
            ),
            1 => (
                format!("○ {} ping response: {}ms", agent, rng.gen_range(50..500)),
                LogLevel::Info,
            ),
            2 => (
                format!("⚠ {} consecutive failures: {}/3", agent, rng.gen_range(1..3)),
                LogLevel::Warning,
            ),
            _ => (
                format!("✗ {} health check failed: timeout after 30s", agent),
                LogLevel::Error,
            ),
        }
    }

    fn hook_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        let issue_id = random_issue_id(rng);
        let agents = ["crew/max", "polecat/Echo", "polecat/Drift"];
        let agent = agents[rng.gen_range(0..agents.len())];

        match rng.gen_range(0..3) {
            0 => (
                format!("✓ Hooked issue {} to {}", issue_id, agent),
                LogLevel::Success,
            ),
            1 => (
                format!("→ Syncing beads for {}...", self.current_rig),
                LogLevel::Action,
            ),
            _ => (
                format!("○ Hook status: {} issues pending", rng.gen_range(1..8)),
                LogLevel::Info,
            ),
        }
    }

    fn mail_event(&mut self, rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
        let senders = ["mayor", "witness", "crew/max", "polecat/Nova"];
        let recipients = ["deacon", "witness", "mayor", "polecat/Spark"];
        let sender = senders[rng.gen_range(0..senders.len())];
        let recipient = recipients[rng.gen_range(0..recipients.len())];

        let subjects = [
            "Work assignment ready",
            "Status update requested",
            "Callback processed",
            "Issue escalation",
            "Convoy dispatch",
        ];
        let subject = subjects[rng.gen_range(0..subjects.len())];

        (
            format!("✉ Mail: {} → {} \"{}\"", sender, recipient, subject),
            LogLevel::Info,
        )
    }
}

fn random_issue_id(rng: &mut rand::rngs::ThreadRng) -> String {
    let prefixes = ["gt", "hq", "api", "fe", "inf"];
    let prefix = prefixes[rng.gen_range(0..prefixes.len())];
    let chars: Vec<char> = "abcdefghijklmnopqrstuvwxyz0123456789".chars().collect();
    let id: String = (0..5).map(|_| chars[rng.gen_range(0..chars.len())]).collect();
    format!("{}-{}", prefix, id)
}
