use vector::Vec3;
use color::Color;
use material::Material;
use transform::Mat4;
use std::rc::Rc;

pub mod sphere;
pub use sphere::*;
pub mod triangle_mesh;
pub use triangle_mesh::*;
pub mod cube;
pub use cube::*;
pub mod difference;
pub use difference::*;

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

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vec3,
    pub color: Color,
}

impl Light {
    pub fn new(position: Vec3, color: Color) -> Light {
        Light { position, color }
    }
}

pub type Uv = (f64, f64);

pub struct Intersection {
    pub distance: f64,
    pub location: Vec3,
    pub normal: Vec3,
    pub uv: Uv,
}

pub struct Hit<'a> {
    pub enter: Option<Intersection>,
    pub exit: Intersection,
    pub object: &'a (SceneObject + 'a),
    pub debug: bool,
}

impl<'a> Hit<'a> {
    pub fn debug(self, debug: bool) -> Hit<'a> {
        Hit {
            enter: self.enter,
            exit: self.exit,
            object: self.object,
            debug,
        }
    }
}

pub trait SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    fn material(&self) -> Rc<Material>;
}
