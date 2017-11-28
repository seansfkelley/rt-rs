pub trait FuzzyEq {
    fn fuzzy_eq(&self, other: Self) -> bool;
}

impl FuzzyEq for f64 {
    fn fuzzy_eq(&self, other: f64) -> bool {
        (self - other).abs() < 1e-10f64
    }
}
