use std::sync::{Mutex, LazyLock};
use rand::Rng;

#[allow(dead_code)]
pub struct LogGenerator;

static LAST_TIMESTAMP: LazyLock<Mutex<f64>> = LazyLock::new(|| Mutex::new(0.0));

impl LogGenerator {
    pub fn timestamp() -> String {
        let mut rng = rand::thread_rng();
        let mut last = LAST_TIMESTAMP.lock().unwrap();
        *last += rng.gen_range(0.01..0.5);
        format!("[{:12.6}]", *last)
    }

    pub fn hex_addr() -> String {
        let mut rng = rand::thread_rng();
        format!("0x{:016x}", rng.gen::<u64>())
    }

    #[allow(dead_code)]
    pub fn version() -> String {
        let mut rng = rand::thread_rng();
        format!(
            "v{}.{}.{}",
            rng.gen_range(1..10),
            rng.gen_range(0..20),
            rng.gen_range(0..50)
        )
    }

    #[allow(dead_code)]
    pub fn progress(current: usize, total: usize) -> String {
        let percentage = (current as f64 / total as f64 * 100.0) as usize;
        format!("{}% ({}/{})", percentage, current, total)
    }
}
