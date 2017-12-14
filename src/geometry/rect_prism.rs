use std::f64;
use core::*;
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
            shading_normal: None,
            uv: (0f64, 0f64), // TODO
            material: None,
        }
    }
}

impl Geometry for RectPrism {
    fn bound(&self) -> BoundingBox {
        BoundingBox {
            min: self.min,
            max: self.max,
        }
    }

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

        if t1 < ray.t_min || t0 > ray.t_max {
            None
        } else if t0 < ray.t_min {
            if t1 <= ray.t_max {
                Some(self.get_intersection(t1, &ray))
            } else {
                None
            }
        } else {
            Some(self.get_intersection(t0, &ray))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref SIMPLE_RECT_PRISM: RectPrism = RectPrism::new(Point::uniform(-1f64), Point::uniform(1f64));
    }

    #[test]
    fn it_should_intersect_a_half_infinite_ray_from_outside() {
        let r = Ray::half_infinite(Point::new(0f64, 0f64, -5f64), Vec3::Z_AXIS);
        let i = SIMPLE_RECT_PRISM.intersect(&r);
        assert!(i.is_some());
        assert_eq!(i.unwrap().distance, 4f64);
    }

    #[test]
    fn it_should_intersect_a_finite_ray_from_outside() {
        let r = Ray::finite(Point::new(0f64, 0f64, -5f64), Vec3::Z_AXIS, 0f64, 5f64);
        let i = SIMPLE_RECT_PRISM.intersect(&r);
        assert!(i.is_some());
        assert_eq!(i.unwrap().distance, 4f64);
    }

    #[test]
    fn it_should_intersect_a_finite_ray_from_inside() {
        let r = Ray::finite(Point::new(0f64, 0f64, 0f64), Vec3::Z_AXIS, 0f64, 5f64);
        let i = SIMPLE_RECT_PRISM.intersect(&r);
        assert!(i.is_some());
        assert_eq!(i.unwrap().distance, 1f64);
    }

    #[test]
    fn it_should_not_intersect_a_half_infinite_ray_from_outside() {
        let r = Ray::half_infinite(Point::new(5f64, 0f64, -5f64), Vec3::Z_AXIS);
        assert!(SIMPLE_RECT_PRISM.intersect(&r).is_none());
    }

    #[test]
    fn it_should_not_intersect_a_finite_ray_from_outside() {
        let r = Ray::finite(Point::new(0f64, 0f64, -5f64), Vec3::Z_AXIS, 0f64, 1f64);
        assert!(SIMPLE_RECT_PRISM.intersect(&r).is_none());
    }

    #[test]
    fn it_should_not_intersect_a_finite_ray_from_inside() {
        let r = Ray::finite(Point::new(0f64, 0f64, 0f64), Vec3::Z_AXIS, 0f64, 0.5f64);
        assert!(SIMPLE_RECT_PRISM.intersect(&r).is_none());
    }
}

