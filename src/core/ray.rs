use math::*;

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
