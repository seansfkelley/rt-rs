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

pub struct CurvePath {
    pub curves: Vec<Box<Curve>>,
}

impl Curve for CurvePath {
    fn at(&self, t: f64) -> Point {
        let number_of_curves = self.curves.len() as f64;
        let scaled_t = t * number_of_curves;
        let floor_t = scaled_t.floor().min(number_of_curves - 1f64);
        let curve = &self.curves[floor_t as usize];
        let curve_t = scaled_t - floor_t;
        curve.at(curve_t)
    }
}

impl CurvePath {
    pub fn from_cubic_bezier_fragments(fragments: Vec<(Point, Point, Point)>, end_point: Point) -> CurvePath {
        let number_of_fragments = fragments.len();
        let mut beziers = Vec::<Box<Curve>>::with_capacity(number_of_fragments);
        for i in 0..(number_of_fragments - 1) {
            let fragment = fragments[i];
            beziers.push(Box::new(CubicBezier {
                p0: fragment.0,
                p1: fragment.1,
                p2: fragment.2,
                p3: fragments[i+1].0,
            }));
        }
        let last_fragment = fragments[number_of_fragments - 1];
        beziers.push(Box::new(CubicBezier {
            p0: last_fragment.0,
            p1: last_fragment.1,
            p2: last_fragment.2,
            p3: end_point,
        }));

        CurvePath { curves: beziers }
    }
}
