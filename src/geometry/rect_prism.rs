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
        const ONE_THIRD: f64 = 1f64 / 3f64;
        const ONE_FOURTH: f64 = 1f64 / 4f64;
        let location = ray.at(t);
        let dimensions = self.max - self.min;
        let (scaled_x, scaled_y, scaled_z) = (
            (location.x - self.min.x) / dimensions.x,
            (location.y - self.min.y) / dimensions.y,
            (location.z - self.min.z) / dimensions.z,
        );

        let (u, v, u_axis, v_axis) =
                   if location.x.fuzzy_eq(self.min.x) {
                (
                    scaled_z * ONE_FOURTH,
                    scaled_y * ONE_THIRD + ONE_THIRD,
                    Vec3::new(0f64, 0f64, dimensions.z),
                    Vec3::new(0f64, dimensions.y, 0f64),
                )
            } else if location.x.fuzzy_eq(self.max.x) {
                (
                    (1f64 - scaled_z) * ONE_FOURTH + 2f64 * ONE_FOURTH,
                    scaled_y * ONE_THIRD + ONE_THIRD,
                    Vec3::new(0f64, 0f64, -dimensions.z),
                    Vec3::new(0f64, dimensions.y, 0f64),
                )
            } else if location.y.fuzzy_eq(self.min.y) {
                (
                    scaled_x * ONE_FOURTH + ONE_FOURTH,
                    scaled_z * ONE_THIRD,
                    Vec3::new(dimensions.x, 0f64, 0f64),
                    Vec3::new(0f64, 0f64, dimensions.z),
                )
            } else if location.y.fuzzy_eq(self.max.y) {
                (
                    scaled_x * ONE_FOURTH + ONE_FOURTH,
                    (1f64 - scaled_z) * ONE_THIRD + 2f64 * ONE_THIRD,
                    Vec3::new(dimensions.x, 0f64, 0f64),
                    Vec3::new(0f64, 0f64, -dimensions.z),
                )
            } else if location.z.fuzzy_eq(self.min.z) {
                (
                    (1f64 - scaled_x) * ONE_FOURTH + 3f64 * ONE_FOURTH,
                    scaled_y * ONE_THIRD + ONE_THIRD,
                    Vec3::new(-dimensions.x, 0f64, 0f64),
                    Vec3::new(0f64, dimensions.y, 0f64),
                )
            } else if location.z.fuzzy_eq(self.max.z) {
                (
                    scaled_x * ONE_FOURTH + ONE_FOURTH,
                    scaled_y * ONE_THIRD + ONE_THIRD,
                    Vec3::new(dimensions.x, 0f64, 0f64),
                    Vec3::new(0f64, dimensions.y, 0f64),
                )
            } else {
                unreachable!();
            };

        Intersection {
            distance: t,
            location,
            geometry: IntersectionGeometry::new(u_axis, v_axis),
            shading_geometry: None,
            uv: Uv(u, v),
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

        foreach_axis!(a in {
            let (mut t_near, mut t_far) = (
                (self.min[a] - ray.origin[a]) / ray.direction[a],
                (self.max[a] - ray.origin[a]) / ray.direction[a],
            );
            if t_near > t_far {
                swap!(t_near, t_far);
            }
            t0 = non_nan_max(t0, t_near);
            t1 = non_nan_min(t1, t_far);
            if t0 > t1 {
                return None;
            }
        });

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

