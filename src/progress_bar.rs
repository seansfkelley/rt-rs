use std::cmp::min;
use std::time::SystemTime;

pub struct ProgressBar {
    total: u32,
    current: u32,
    title: String,
    start_timestamp: Option<SystemTime>,
}

impl ProgressBar {
    pub fn new(total: u32) -> ProgressBar {
        ProgressBar {
            total,
            current: 0,
            title: "".to_owned(),
            start_timestamp: None,
        }
    }

    pub fn mark_start(&mut self) {
        match self.start_timestamp {
            Some(_) => { panic!() },
            None => { self.start_timestamp = Some(SystemTime::now()) },
        }
    }

    pub fn increment(&mut self, count: u32) {
        self.current = min(self.current + count, self.total)
    }

    pub fn is_complete(&self) -> bool {
        self.current == self.total
    }

    pub fn set_title(&mut self, title: String) {
        self.title = title
    }

    pub fn render(&self) {
        // TODO: Actual progress bar and overwrite current line.
        let fraction_complete = self.current as f64 / self.total as f64;
        let eta_string: String = self.start_timestamp
            .map(|start| {
                let duration = start.elapsed().unwrap();
                let elapsed_time = (duration.as_secs() as f64 * 1e9f64 + duration.subsec_nanos() as f64) / 1e9f64;
                let eta = elapsed_time / fraction_complete - elapsed_time;
                if eta.is_finite() {
                    format!(", eta: {:3.1}s", eta)
                } else {
                    "".to_owned()
                }
            })
            .unwrap_or("".to_owned());
        println!("{} ({:3.1}%{})",
            self.title,
            100f64 * self.current as f64 / self.total as f64,
            eta_string,
        );
    }
}
