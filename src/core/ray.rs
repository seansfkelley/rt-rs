use math::Vec3;
use transform::Mat4;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        direction.assert_normalized();
        Ray { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Vec3 {
        self.origin + self.direction * distance
    }

    pub fn transform(&self, transform: Mat4, transform_without_scale: Mat4) -> Ray {
        let origin = transform * self.origin;
        let direction = (transform_without_scale * self.direction).as_unit_vector();
        Ray { origin, direction }
    }
}
