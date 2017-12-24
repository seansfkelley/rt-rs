use std::io::{ stderr, Write };
use std::time::SystemTime;
use std::sync::atomic::{ AtomicUsize, Ordering };
use terminal_size::{ terminal_size, Width };

pub struct ProgressBar {
    operations_total: usize,
    operations_current: AtomicUsize,
    frame_total: usize,
    frame_current: AtomicUsize,
    start_timestamp: SystemTime,
}

impl ProgressBar {
    pub fn new(operations_total: usize, frame_total: usize) -> ProgressBar {
        ProgressBar {
            operations_total,
            operations_current: AtomicUsize::new(0),
            frame_total,
            frame_current: AtomicUsize::new(0),
            start_timestamp: SystemTime::now(),
        }
    }

    pub fn increment_operations(&self, count: usize) {
        self.operations_current.fetch_add(count, Ordering::Relaxed);
    }

    pub fn increment_frame(&self) {
        self.frame_current.fetch_add(1, Ordering::Relaxed);
    }

    pub fn is_complete(&self) -> bool {
        self.operations_current.load(Ordering::SeqCst) == self.operations_total &&
        self.frame_current.load(Ordering::SeqCst) == self.frame_total
    }

    fn elapsed_time_seconds(&self) -> f64 {
        let duration = self.start_timestamp.elapsed().unwrap();
        (duration.as_secs() as f64 * 1e9f64 + duration.subsec_nanos() as f64) / 1e9f64
    }

    pub fn render(&self) {
        let fraction_complete = self.operations_current.load(Ordering::Relaxed) as f64 / self.operations_total as f64;
        let elapsed_time = self.elapsed_time_seconds();
        let eta = {
            let eta = elapsed_time / fraction_complete - elapsed_time;
            if eta.is_finite() { eta } else { 0f64 }
        };

        let prefix = format!("frame {}/{} - {:3.1}% - {:.1}s [",
            self.frame_current.load(Ordering::Relaxed),
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
                eprint!("\r{}{}{}{}", prefix, "=".repeat(fill_width), " ".repeat(empty_width), suffix);
                stderr().flush().ok().unwrap();
            },
            None => {},
        }
    }
}
