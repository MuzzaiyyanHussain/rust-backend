use std::collections::HashMap;
use std::time::{Instant, Duration};
use std::sync::{Mutex, Arc};

#[derive(Clone)]
pub struct RateLimiter {
    pub store: Arc<Mutex<HashMap<String, (u32, Instant)>>>,
    pub limit: u32,
    pub window: Duration,
}

impl RateLimiter {
    pub fn new(limit: u32, window_secs: u64) -> Self {
        Self {
            store: Arc::new(Mutex::new(HashMap::new())),
            limit,
            window: Duration::from_secs(window_secs),
        }
    }

    pub fn check(&self, key: String) -> bool {
        let mut store = self.store.lock().unwrap();
        let now = Instant::now();

        let entry = store.entry(key).or_insert((0, now));

        if now.duration_since(entry.1) > self.window {
            *entry = (1, now);
            return true;
        }

        if entry.0 < self.limit {
            entry.0 += 1;
            return true;
        }

        false
    }
}