use math::*;
use super::transform::{ Transform, Transformable };

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Point, direction: Vec3) -> Ray {
        direction.assert_normalized();
        Ray { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Point {
        self.origin + self.direction * distance
    }
}

impl Transformable for Ray {
    fn transform(&self, transform: &Transform) -> Ray {
        Ray {
            origin: self.origin.transform(transform),
            direction: self.direction.transform(transform),
        }
    }

    fn invert_transform(&self, transform: &Transform) -> Ray {
        Ray {
            origin: self.origin.invert_transform(transform),
            direction: self.direction.invert_transform(transform),
        }
    }
}
