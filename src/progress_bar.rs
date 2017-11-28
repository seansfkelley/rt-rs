use std::io::{ stdout, Write };
use std::cmp::min;
use std::time::SystemTime;
use terminal_size::{ terminal_size, Width };

pub struct ProgressBar {
    operations_total: u32,
    operations_current: u32,
    frame_total: u32,
    frame_current: u32,
    start_timestamp: SystemTime,
}

impl ProgressBar {
    pub fn new(operations_total: u32, frame_total: u32) -> ProgressBar {
        ProgressBar {
            operations_total,
            operations_current: 0,
            frame_total,
            frame_current: 0,
            start_timestamp: SystemTime::now(),
        }
    }

    pub fn increment_operations(&mut self, count: u32) {
        self.operations_current = min(self.operations_current + count, self.operations_total)
    }

    pub fn increment_frame(&mut self) {
        self.frame_current = min(self.frame_current + 1, self.frame_total)
    }

    pub fn is_complete(&self) -> bool {
        self.operations_current == self.operations_total && self.frame_current == self.frame_total
    }

    fn elapsed_time_seconds(&self) -> f64 {
        let duration = self.start_timestamp.elapsed().unwrap();
        (duration.as_secs() as f64 * 1e9f64 + duration.subsec_nanos() as f64) / 1e9f64
    }

    pub fn render(&self) {
        let fraction_complete = self.operations_current as f64 / self.operations_total as f64;
        let elapsed_time = self.elapsed_time_seconds();
        let eta = {
            let eta = elapsed_time / fraction_complete - elapsed_time;
            if eta.is_finite() { eta } else { 0f64 }
        };

        let prefix = format!("frame {}/{} - {:3.1}% - {:.1}s [",
            self.frame_current,
            self.frame_total,
            100f64 * fraction_complete,
            elapsed_time,
        );
        let suffix = format!("] {:.1}s", eta);
        match terminal_size() {
            Some((Width(width), _)) => {
                let total_bar_width = (width as usize) - prefix.len() - suffix.len();
                let fill_width = (total_bar_width as f64 * fraction_complete).floor() as usize;
                let empty_width = total_bar_width - fill_width;
                print!("\r{}{}{}{}", prefix, "=".repeat(fill_width), " ".repeat(empty_width), suffix);
                stdout().flush().ok().unwrap();
            },
            None => {},
        }
    }
}
