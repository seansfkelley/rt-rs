use std::f64;
use core::*;
use geometry::Geometry;
use math::*;

macro_rules! swap {
    ($a:ident, $b:ident) => {
        let temp = $a;
        $a = $b;
        $b = temp;
    };
}

#[derive(Debug)]
pub struct RectPrism {
    min: Point,
    max: Point,
}

impl RectPrism {
    pub fn new(min: Point, max: Point) -> RectPrism {
        RectPrism { min, max }
    }

    fn get_intersection(&self, t: f64, ray: &Ray) -> Intersection {
        let location = ray.at(t);

        let normal =
                   if location.x.fuzzy_eq(self.min.x) {
                Normal::new(-1f64, 0f64, 0f64)
            } else if location.x.fuzzy_eq(self.max.x) {
                Normal::new(1f64, 0f64, 0f64)
            } else if location.y.fuzzy_eq(self.min.y) {
                Normal::new(0f64, -1f64, 0f64)
            } else if location.y.fuzzy_eq(self.max.y) {
                Normal::new(0f64, 1f64, 0f64)
            } else if location.z.fuzzy_eq(self.min.z) {
                Normal::new(0f64, 0f64, -1f64)
            } else if location.z.fuzzy_eq(self.max.z) {
                Normal::new(0f64, 0f64, 1f64)
            } else {
                unreachable!();
            };

        Intersection {
            distance: t,
            location,
            normal,
            uv: (0f64, 0f64), // TODO
        }
    }
}

impl Geometry for RectPrism {
    // pbrt pg. 194
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let (mut t0, mut t1) = (f64::NEG_INFINITY, f64::INFINITY);

        for i in 0..3 {
            let (mut t_near, mut t_far) = (
                (self.min[i] - ray.origin[i]) / ray.direction[i],
                (self.max[i] - ray.origin[i]) / ray.direction[i],
            );
            if t_near > t_far {
                swap!(t_near, t_far);
            }
            t0 = non_nan_max(t0, t_near);
            t1 = non_nan_min(t1, t_far);
            if t0 > t1 {
                return None;
            }
        }

        if t1 < 0f64 {
            None
        } else if t0 < 0f64 {
            Some(self.get_intersection(t1, &ray))
        } else {
            Some(self.get_intersection(t0, &ray))
        }
    }
}

impl Boundable for RectPrism {
    fn bound(&self) -> BoundingBox {
        BoundingBox {
            min: self.min,
            max: self.max,
        }
    }
}
