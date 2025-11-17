use rand::seq::SliceRandom;
use std::fs;
use std::path::Path;

/// Manages build log messages for authentic compilation output
pub struct BuildLogs {
    logs: Vec<String>,
}

impl BuildLogs {
    /// Load build logs from file or use defaults
    pub fn load() -> Self {
        let log_path = Path::new("data/build.log");

        let logs = if log_path.exists() {
            match fs::read_to_string(log_path) {
                Ok(content) => {
                    content
                        .lines()
                        .filter(|line| !line.trim().is_empty())
                        .map(String::from)
                        .collect()
                }
                Err(_) => Self::default_logs(),
            }
        } else {
            Self::default_logs()
        };

        Self { logs }
    }

    /// Get multiple random build log messages
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
            "  CC      kernel/fork.o".to_string(),
            "  CC      kernel/exec_domain.o".to_string(),
            "  CC      kernel/panic.o".to_string(),
            "  CC      kernel/cpu.o".to_string(),
            "  CC      kernel/exit.o".to_string(),
            "  CC      kernel/softirq.o".to_string(),
            "  CC      kernel/resource.o".to_string(),
            "  CC      kernel/sysctl.o".to_string(),
            "  CC      kernel/capability.o".to_string(),
            "  CC      kernel/ptrace.o".to_string(),
            "  CC      kernel/signal.o".to_string(),
            "  CC      kernel/sys.o".to_string(),
            "  CC      kernel/umh.o".to_string(),
            "  CC      kernel/workqueue.o".to_string(),
            "  CC      kernel/pid.o".to_string(),
            "  CC      kernel/task_work.o".to_string(),
            "  CC      kernel/extable.o".to_string(),
            "  CC      kernel/params.o".to_string(),
            "  CC      kernel/kthread.o".to_string(),
            "  CC      kernel/sys_ni.o".to_string(),
            "  CC      kernel/nsproxy.o".to_string(),
            "  CC      kernel/notifier.o".to_string(),
            "  CC      kernel/ksysfs.o".to_string(),
            "  CC      kernel/cred.o".to_string(),
            "  CC      kernel/reboot.o".to_string(),
            "  CC      kernel/async.o".to_string(),
            "  CC      kernel/range.o".to_string(),
            "  CC      kernel/smpboot.o".to_string(),
            "  CC      kernel/ucount.o".to_string(),
            "  CC      mm/filemap.o".to_string(),
            "  CC      mm/mempool.o".to_string(),
            "  CC      mm/oom_kill.o".to_string(),
            "  CC      mm/maccess.o".to_string(),
            "  CC      mm/page_alloc.o".to_string(),
            "  CC      mm/page-writeback.o".to_string(),
            "  CC      mm/readahead.o".to_string(),
            "  CC      mm/swap.o".to_string(),
            "  CC      mm/truncate.o".to_string(),
            "  CC      mm/vmscan.o".to_string(),
            "  CC      mm/shmem.o".to_string(),
            "  CC      mm/util.o".to_string(),
            "  CC      mm/mmzone.o".to_string(),
            "  CC      mm/vmstat.o".to_string(),
            "  CC      fs/open.o".to_string(),
            "  CC      fs/read_write.o".to_string(),
            "  CC      fs/file_table.o".to_string(),
            "  CC      fs/super.o".to_string(),
            "  CC      fs/char_dev.o".to_string(),
            "  CC      fs/stat.o".to_string(),
            "  CC      fs/exec.o".to_string(),
            "  CC      fs/pipe.o".to_string(),
            "  CC      fs/namei.o".to_string(),
            "  CC      fs/fcntl.o".to_string(),
            "  CC      fs/ioctl.o".to_string(),
            "  CC      fs/readdir.o".to_string(),
            "  CC      fs/select.o".to_string(),
            "  CC      fs/dcache.o".to_string(),
            "  CC      fs/inode.o".to_string(),
            "  CC      fs/attr.o".to_string(),
            "  CC      fs/bad_inode.o".to_string(),
            "  CC      fs/file.o".to_string(),
            "  CC      fs/filesystems.o".to_string(),
            "  CC      fs/namespace.o".to_string(),
            "  CC      fs/seq_file.o".to_string(),
            "  CC      fs/xattr.o".to_string(),
            "  CC      fs/libfs.o".to_string(),
            "  CC      fs/fs-writeback.o".to_string(),
            "  CC      fs/pnode.o".to_string(),
            "  CC      fs/splice.o".to_string(),
            "  CC      fs/sync.o".to_string(),
            "  CC      fs/utimes.o".to_string(),
            "  CC      fs/d_path.o".to_string(),
            "  CC      fs/stack.o".to_string(),
            "  CC      fs/fs_struct.o".to_string(),
            "  CC      fs/statfs.o".to_string(),
            "  CC      fs/fs_pin.o".to_string(),
            "  CC      fs/nsfs.o".to_string(),
            "  CC      fs/fs_types.o".to_string(),
            "  CC      fs/fs_context.o".to_string(),
            "  CC      fs/fs_parser.o".to_string(),
            "  CC      fs/fsopen.o".to_string(),
            "  LD      fs/built-in.a".to_string(),
            "  LD      mm/built-in.a".to_string(),
            "  LD      kernel/built-in.a".to_string(),
            "  AR      vmlinux.a".to_string(),
            "  LD      vmlinux.o".to_string(),
        ]
    }
}

impl Default for BuildLogs {
    fn default() -> Self {
        Self::load()
    }
}
