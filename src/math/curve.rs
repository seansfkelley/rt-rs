use math::xyz::Point;

pub trait Curve {
    // t: 0-1
    fn at(&self, t: f64) -> Point;
}

pub struct CubicBezier {
    pub p0: Point,
    pub p1: Point,
    pub p2: Point,
    pub p3: Point,
}

impl Curve for CubicBezier {
    fn at(&self, t: f64) -> Point {
        self.p0 * (1f64 - t).powi(3)
            + self.p1 * 3f64 * (1f64 - t).powi(2) * t
            + self.p2 * 3f64 * (1f64 - t) * t.powi(2)
            + self.p3 * t.powi(3)
    }
}

#[allow(dead_code)]
pub struct Path {
    curves: Vec<Box<Curve>>,
}

impl Curve for Path {
    fn at(&self, t: f64) -> Point {
        let scaled_t = t * self.curves.len() as f64;
        let floor_t = scaled_t.floor();
        let curve = &self.curves[floor_t as usize];
        let curve_t = scaled_t - floor_t;
        curve.at(curve_t)
    }
}
