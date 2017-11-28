pub fn non_nan_min(a: f64, b: f64) -> f64 {
    if a < b { a } else { b }
}

pub fn non_nan_max(a: f64, b: f64) -> f64 {
    if a > b { a } else { b }
}
