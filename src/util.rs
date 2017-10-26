pub trait Clamp {
    fn clamp(self, min: Self, max: Self) -> Self;
}

impl Clamp for f64 {
    fn clamp(self, min: f64, max: f64) -> f64 {
        self.min(max).max(min)
    }
}
