use super::InstallationStage;
use crate::log_generator::LogGenerator;
use crate::ui::Spinner;
use colored::*;
use rand::Rng;
use std::io;
use std::thread;
use std::time::Duration;

pub struct TrafficStage;

impl TrafficStage {
    pub fn new() -> Self {
        Self
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

        spinner.animate("Loading traffic generators...", 1200, exit_check)?;

        let generators: Vec<(&str, fn(&mut rand::rngs::ThreadRng) -> (String, LogLevel))> = vec![
            ("OpenSSH", generate_openssh_log),
            ("HDFS", generate_hdfs_log),
            ("Apache", generate_apache_log),
            ("Proxifier", generate_proxifier_log),
            ("Linux", generate_linux_log),
            ("Zookeeper", generate_zookeeper_log),
        ];

        println!(
            "{} Loaded {} log generators: {}",
            LogGenerator::timestamp().dimmed(),
            generators.len().to_string().cyan(),
            generators
                .iter()
                .map(|(name, _)| *name)
                .collect::<Vec<_>>()
                .join(", ")
                .dimmed()
        );
        thread::sleep(Duration::from_millis(300));

        println!(
            "{} Starting network traffic simulation...",
            LogGenerator::timestamp().dimmed()
        );
        println!();

        let lines_to_show = rng.gen_range(80..150);
        let mut shown = 0;
        let mut current_gen_idx = rng.gen_range(0..generators.len());

        while shown < lines_to_show {
            if exit_check() {
                return Err(io::Error::new(io::ErrorKind::Interrupted, "User interrupt"));
            }

            if rng.gen_bool(0.12) {
                current_gen_idx = rng.gen_range(0..generators.len());
                if rng.gen_bool(0.3) {
                    println!(
                        "{} {} {}",
                        LogGenerator::timestamp().dimmed(),
                        "--- switching to".dimmed(),
                        generators[current_gen_idx].0.cyan()
                    );
                    thread::sleep(Duration::from_millis(200));
                }
            }

            let (name, generator) = &generators[current_gen_idx];
            let (log_line, level) = generator(&mut rng);

            let prefix = format!("[{}]", name);
            let truncated = truncate_line(&log_line, 100);

            let colored_line = match level {
                LogLevel::Error => truncated.red(),
                LogLevel::Warning => truncated.yellow(),
                LogLevel::Success => truncated.green(),
                LogLevel::Alert => truncated.bright_red(),
                LogLevel::Info => truncated.dimmed(),
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

#[derive(Clone, Copy)]
enum LogLevel {
    Info,
    Warning,
    Error,
    Success,
    Alert,
}

fn truncate_line(line: &str, max_width: usize) -> String {
    if line.len() > max_width {
        format!("{}...", &line[..max_width - 3])
    } else {
        line.to_string()
    }
}

fn random_ip(rng: &mut rand::rngs::ThreadRng) -> String {
    format!(
        "{}.{}.{}.{}",
        rng.gen_range(1..255),
        rng.gen_range(0..255),
        rng.gen_range(0..255),
        rng.gen_range(1..255)
    )
}

fn random_port(rng: &mut rand::rngs::ThreadRng) -> u16 {
    rng.gen_range(1024..65535)
}

fn random_pid(rng: &mut rand::rngs::ThreadRng) -> u32 {
    rng.gen_range(1000..50000)
}

fn syslog_timestamp(rng: &mut rand::rngs::ThreadRng) -> String {
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
    format!(
        "{} {:2} {:02}:{:02}:{:02}",
        months[rng.gen_range(0..12)],
        rng.gen_range(1..29),
        rng.gen_range(0..24),
        rng.gen_range(0..60),
        rng.gen_range(0..60)
    )
}

fn generate_openssh_log(rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
    let hostnames = ["LabSZ", "server01", "prod-web", "gateway", "bastion"];
    let users = ["root", "admin", "webmaster", "test", "oracle", "postgres", "jenkins", "deploy"];
    let domains = [
        "ns.marryaldkfaczcz.com",
        "scanner.security.net",
        "bot.malware.ru",
        "ec2-52-80-34-196.compute.amazonaws.com",
        "dynamic-dsl-ip.provider.net",
    ];

    let hostname = hostnames[rng.gen_range(0..hostnames.len())];
    let pid = random_pid(rng);
    let ip = random_ip(rng);
    let port = random_port(rng);

    let (message, level) = match rng.gen_range(0..10) {
        0 => {
            let domain = domains[rng.gen_range(0..domains.len())];
            (
                format!("reverse mapping checking getaddrinfo for {} [{}] failed - POSSIBLE BREAK-IN ATTEMPT!", domain, ip),
                LogLevel::Alert,
            )
        }
        1 | 2 => {
            let user = users[rng.gen_range(0..users.len())];
            (
                format!("Invalid user {} from {}", user, ip),
                LogLevel::Alert,
            )
        }
        3 | 4 => {
            let user = users[rng.gen_range(0..users.len())];
            (
                format!("Failed password for invalid user {} from {} port {} ssh2", user, ip, port),
                LogLevel::Warning,
            )
        }
        5 => (
            format!("Connection closed by {} [preauth]", ip),
            LogLevel::Info,
        ),
        6 => (
            format!("Received disconnect from {}: 11: Bye Bye [preauth]", ip),
            LogLevel::Info,
        ),
        7 => (
            format!("Disconnecting: Too many authentication failures for root [preauth]"),
            LogLevel::Warning,
        ),
        8 => {
            let user = users[rng.gen_range(0..users.len())];
            (
                format!("Accepted publickey for {} from {} port {} ssh2", user, ip, port),
                LogLevel::Success,
            )
        }
        _ => (
            format!("pam_unix(sshd:auth): authentication failure; logname= uid=0 euid=0 tty=ssh ruser= rhost={}", ip),
            LogLevel::Warning,
        ),
    };

    (format!("{} {} sshd[{}]: {}", syslog_timestamp(rng), hostname, pid, message), level)
}

fn generate_hdfs_log(rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
    let block_id: i64 = rng.gen_range(-9000000000000000000i64..9000000000000000000i64);
    let ip = format!("10.{}.{}.{}", rng.gen_range(250..252), rng.gen_range(0..255), rng.gen_range(1..255));
    let size: u64 = 67108864;
    let thread_id = rng.gen_range(100..2000);

    let date = format!(
        "{:02}{:02}{:02} {:02}{:02}{:02}",
        rng.gen_range(8..12),
        rng.gen_range(1..13),
        rng.gen_range(1..29),
        rng.gen_range(0..24),
        rng.gen_range(0..60),
        rng.gen_range(0..60)
    );

    let (message, level) = match rng.gen_range(0..8) {
        0 => (
            format!("INFO dfs.DataNode$PacketResponder: PacketResponder {} for block blk_{} terminating", rng.gen_range(0..3), block_id.abs()),
            LogLevel::Info,
        ),
        1 => (
            format!("INFO dfs.FSNamesystem: BLOCK* NameSystem.addStoredBlock: blockMap updated: {}:50010 is added to blk_{} size {}", ip, block_id.abs(), size),
            LogLevel::Info,
        ),
        2 => (
            format!("INFO dfs.DataNode$PacketResponder: Received block blk_{} of size {} from /{}", block_id.abs(), size, ip),
            LogLevel::Success,
        ),
        3 => (
            format!("INFO dfs.DataNode$DataXceiver: Receiving block blk_{} src: /{}:{} dest: /{}:50010", block_id.abs(), ip, random_port(rng), ip),
            LogLevel::Info,
        ),
        4 => (
            format!("INFO dfs.FSNamesystem: BLOCK* NameSystem.allocateBlock: /user/root/rand/_temporary/_task_{}. blk_{}", rng.gen_range(100000..999999), block_id.abs()),
            LogLevel::Info,
        ),
        5 => (
            format!("INFO dfs.DataBlockScanner: Verification succeeded for blk_{}", block_id.abs()),
            LogLevel::Success,
        ),
        6 => (
            format!("WARN dfs.DataNode: Slow BlockReceiver write data to disk cost: {}ms", rng.gen_range(300..5000)),
            LogLevel::Warning,
        ),
        _ => (
            format!("ERROR dfs.DataNode: Exception in receiveBlock for blk_{}", block_id.abs()),
            LogLevel::Error,
        ),
    };

    (format!("{} {} {}", date, thread_id, message), level)
}

fn generate_apache_log(rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
    let days = ["Sun", "Mon", "Tue", "Wed", "Thu", "Fri", "Sat"];
    let months = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];

    let timestamp = format!(
        "{} {} {:02} {:02}:{:02}:{:02} {:04}",
        days[rng.gen_range(0..7)],
        months[rng.gen_range(0..12)],
        rng.gen_range(1..29),
        rng.gen_range(0..24),
        rng.gen_range(0..60),
        rng.gen_range(0..60),
        rng.gen_range(2020..2026)
    );

    let child_id = rng.gen_range(6700..6800);
    let slot = rng.gen_range(6..11);

    let (level_str, message, level) = match rng.gen_range(0..6) {
        0 => ("notice", "workerEnv.init() ok /etc/httpd/conf/workers2.properties".to_string(), LogLevel::Success),
        1 => ("error", format!("mod_jk child workerEnv in error state {}", rng.gen_range(6..8)), LogLevel::Error),
        2 => ("notice", format!("jk2_init() Found child {} in scoreboard slot {}", child_id, slot), LogLevel::Info),
        3 => ("warn", "long lost child came home!".to_string(), LogLevel::Warning),
        4 => ("notice", "Apache/2.4.41 configured -- resuming normal operations".to_string(), LogLevel::Success),
        _ => ("error", format!("(28)No space left on device: Cannot create SSLMutex"), LogLevel::Error),
    };

    (format!("[{}] [{}] {}", timestamp, level_str, message), level)
}

fn generate_proxifier_log(rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
    let processes = ["chrome.exe", "firefox.exe", "slack.exe", "code.exe", "node.exe", "python.exe", "curl.exe"];
    let proxies = [
        "proxy.corp.local:8080",
        "squid.internal:3128",
        "proxy.cse.cuhk.edu.hk:5070",
        "forward.gateway.net:8888",
    ];

    let process = processes[rng.gen_range(0..processes.len())];
    let proxy = proxies[rng.gen_range(0..proxies.len())];
    let bytes_sent: u64 = rng.gen_range(100..50000);
    let bytes_recv: u64 = rng.gen_range(100..100000);

    let timestamp = format!(
        "{:02}.{:02} {:02}:{:02}:{:02}",
        rng.gen_range(1..13),
        rng.gen_range(1..29),
        rng.gen_range(0..24),
        rng.gen_range(0..60),
        rng.gen_range(0..60)
    );

    let (message, level) = match rng.gen_range(0..5) {
        0 => (
            format!("{} - {} open through proxy {} HTTPS", process, proxy, proxy),
            LogLevel::Info,
        ),
        1 => (
            format!("{} - {} close, {} bytes sent, {} bytes received, lifetime <1 sec", process, proxy, bytes_sent, bytes_recv),
            LogLevel::Info,
        ),
        2 => (
            format!("{} - {} close, {} bytes ({:.2} KB) sent, {} bytes ({:.2} KB) received, lifetime {:02}:{:02}",
                process, proxy, bytes_sent, bytes_sent as f64 / 1024.0, bytes_recv, bytes_recv as f64 / 1024.0,
                rng.gen_range(0..5), rng.gen_range(0..60)),
            LogLevel::Success,
        ),
        3 => (
            format!("{} - {} error: Connection refused", process, proxy),
            LogLevel::Error,
        ),
        _ => (
            format!("{} - {} close, 0 bytes sent, 0 bytes received, lifetime 00:01", process, proxy),
            LogLevel::Warning,
        ),
    };

    (format!("[{}] {}", timestamp, message), level)
}

fn generate_linux_log(rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
    let hostnames = ["combo", "server", "workstation", "node01", "gateway"];
    let services = ["sshd", "sudo", "su", "cron", "systemd", "kernel"];
    let users = ["root", "admin", "cyrus", "news", "www-data", "nobody"];

    let hostname = hostnames[rng.gen_range(0..hostnames.len())];
    let service = services[rng.gen_range(0..services.len())];
    let pid = random_pid(rng);
    let ip = random_ip(rng);

    let (message, level) = match rng.gen_range(0..8) {
        0 | 1 => (
            format!("{}(pam_unix)[{}]: authentication failure; logname= uid=0 euid=0 tty=NODEVssh ruser= rhost={}", service, pid, ip),
            LogLevel::Warning,
        ),
        2 => (
            format!("{}(pam_unix)[{}]: check pass; user unknown", service, pid),
            LogLevel::Warning,
        ),
        3 => {
            let user = users[rng.gen_range(0..users.len())];
            (
                format!("{}(pam_unix)[{}]: session opened for user {} by (uid=0)", service, pid, user),
                LogLevel::Success,
            )
        }
        4 => {
            let user = users[rng.gen_range(0..users.len())];
            (
                format!("{}(pam_unix)[{}]: session closed for user {}", service, pid, user),
                LogLevel::Info,
            )
        }
        5 => (
            "logrotate: ALERT exited abnormally with [1]".to_string(),
            LogLevel::Alert,
        ),
        6 => (
            format!("kernel: [{}] TCP: request_sock_TCP: Possible SYN flooding on port {}.", rng.gen_range(10000..99999), rng.gen_range(80..8080)),
            LogLevel::Alert,
        ),
        _ => (
            format!("systemd[1]: Started Session {} of user root.", rng.gen_range(100..999)),
            LogLevel::Info,
        ),
    };

    (format!("{} {} {}", syslog_timestamp(rng), hostname, message), level)
}

fn generate_zookeeper_log(rng: &mut rand::rngs::ThreadRng) -> (String, LogLevel) {
    let ips = ["10.10.34.11", "10.10.34.12", "10.10.34.13", "10.10.34.14"];
    let ip = ips[rng.gen_range(0..ips.len())];
    let port = rng.gen_range(40000..60000);
    let node_id: u64 = rng.gen_range(100000000000u64..999999999999u64);

    let timestamp = format!(
        "{:04}-{:02}-{:02} {:02}:{:02}:{:02},{:03}",
        rng.gen_range(2020..2026),
        rng.gen_range(1..13),
        rng.gen_range(1..29),
        rng.gen_range(0..24),
        rng.gen_range(0..60),
        rng.gen_range(0..60),
        rng.gen_range(0..1000)
    );

    let (level_str, component, message, level) = match rng.gen_range(0..8) {
        0 => (
            "INFO ",
            "QuorumPeer[myid=1]/0:0:0:0:0:0:0:0:2181:FastLeaderElection@774".to_string(),
            format!("Notification time out: {}", rng.gen_range(1000..10000)),
            LogLevel::Info,
        ),
        1 => (
            "INFO ",
            format!("/{}:3888:QuorumCnxManager$Listener@493", ip),
            format!("Received connection request /{}:{}", ip, port),
            LogLevel::Info,
        ),
        2 | 3 => (
            "WARN ",
            format!("SendWorker:{}:QuorumCnxManager$SendWorker@688", node_id),
            "Send worker leaving thread".to_string(),
            LogLevel::Warning,
        ),
        4 => (
            "WARN ",
            format!("SendWorker:{}:QuorumCnxManager$SendWorker@679", node_id),
            "Interrupted while waiting for message on queue".to_string(),
            LogLevel::Warning,
        ),
        5 => (
            "WARN ",
            format!("RecvWorker:{}:QuorumCnxManager$RecvWorker@762", node_id),
            format!("Connection broken for id {}, my id = 1, error = ", node_id),
            LogLevel::Warning,
        ),
        6 => (
            "WARN ",
            format!("RecvWorker:{}:QuorumCnxManager$RecvWorker@765", node_id),
            "Interrupting SendWorker".to_string(),
            LogLevel::Warning,
        ),
        _ => (
            "INFO ",
            "NIOServerCnxn@1001".to_string(),
            format!("Closed socket connection for client /{}:{}", ip, port),
            LogLevel::Info,
        ),
    };

    (format!("{} - {} [{}] - {}", timestamp, level_str, component, message), level)
}
