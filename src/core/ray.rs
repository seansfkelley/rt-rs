use std::f64;
use math::*;
use super::transform::{ Transform, Transformable };

#[derive(Debug, Clone)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
    pub t_min: f64,
    pub t_max: f64,
}

impl Ray {
    pub fn half_infinite(origin: Point, direction: Vec3) -> Ray {
        direction.assert_normalized();
        Ray {
            origin,
            direction,
            t_min: 0f64,
            t_max: f64::INFINITY,
        }
    }

    #[cfg(test)]
    pub fn finite(origin: Point, direction: Vec3, t_min: f64, t_max: f64) -> Ray {
        direction.assert_normalized();
        Ray {
            origin,
            direction,
            t_min,
            t_max,
        }
    }

    pub fn at(&self, t: f64) -> Point {
        assert!(t >= self.t_min);
        assert!(t <= self.t_max);
        self.origin + self.direction * t
    }

    pub fn with_min(self, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction,
            t_min: t,
            t_max: self.t_max,
        }
    }

    pub fn with_max(self, t: f64) -> Ray {
        Ray {
            origin: self.origin,
            direction: self.direction,
            t_min: self.t_min,
            t_max: t,
        }
    }
}

impl Transformable for Ray {
    fn transform(&self, transform: &Transform) -> Ray {
        Ray {
            origin: self.origin.transform(transform),
            direction: self.direction.transform(transform),
            t_min: self.t_min,
            t_max: self.t_max,
        }
    }

    fn invert_transform(&self, transform: &Transform) -> Ray {
        Ray {
            origin: self.origin.invert_transform(transform),
            direction: self.direction.invert_transform(transform),
            t_min: self.t_min,
            t_max: self.t_max,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_create_a_half_infinitely_long_ray() {
        let r = Ray::half_infinite(Point::uniform(0f64), Vec3::X_AXIS);
        assert_eq!(r.t_min, 0f64);
        assert_eq!(r.t_max, f64::INFINITY);
    }

    #[test]
    fn it_should_create_a_finite_ray() {
        let r = Ray::finite(Point::uniform(0f64), Vec3::X_AXIS, 0f64, 1f64);
        assert_eq!(r.t_min, 0f64);
        assert_eq!(r.t_max, 1f64);
    }

    #[test]
    #[should_panic]
    fn it_should_throw_if_the_direction_is_not_normalized() {
        Ray::half_infinite(Point::uniform(0f64), Vec3::uniform(1f64));
    }

    #[test]
    #[should_panic]
    fn it_should_throw_if_calling_at_with_out_of_bounds_t() {
        let r = Ray::half_infinite(Point::uniform(0f64), Vec3::X_AXIS);
        r.at(-1f64);
    }
}
