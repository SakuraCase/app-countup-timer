use std::time::{Duration, Instant};
pub struct CountUpTimer {
    start_time: Option<Instant>,
    elapsed: Duration,
    history: Vec<Duration>,
}

impl CountUpTimer {
    pub fn new() -> Self {
        CountUpTimer {
            start_time: None,
            elapsed: Duration::new(0, 0),
            history: Vec::new(),
        }
    }

    pub fn start(&mut self) {
        if self.start_time.is_none() {
            self.start_time = Some(Instant::now());
        }
    }

    pub fn stop(&mut self) {
        if let Some(start_time) = self.start_time {
            self.elapsed += start_time.elapsed();
            self.start_time = None;
            self.history.push(self.elapsed);

            if self.history.len() > 10 {
                self.history.remove(0);
            }

            self.elapsed = Duration::new(0, 0);
        }
    }

    pub fn get_history_last(&self) -> String {
        self.get_history(self.history.len() - 1)
    }

    pub fn get_history(&self, index: usize) -> String {
        if index < self.history.len() {
            let duration = &self.history[index];
            let secs = duration.as_secs();
            let minutes = secs / 60;
            let seconds = secs % 60;
            format!("{}:{:02}", minutes, seconds)
        } else {
            "Invalid history index".to_string()
        }
    }
}
