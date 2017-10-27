use vector::Vec3;
use color::Color;
use material::Material;
use util::Clamp;
use std::f64::consts::PI;
use std::rc::Rc;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        assert!((direction.magnitude() - 1f64).abs() < 1e-10);
        Ray { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Vec3 {
        self.origin + self.direction * distance
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

pub struct Intersection<'a> {
    pub distance: f64,
    pub location: Vec3,
    pub normal: Vec3,
    pub uv: (f64, f64),
    pub object: &'a (SceneObject + 'a),
}

pub trait SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
    fn material(&self) -> Rc<Material>;
}

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Rc<Material>) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl SceneObject for Sphere {
    // TODO: Verify this implementation against pbrt.
    // TODO: Should transform ray into world space first so the rest of the math is easy.
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let l = self.center - ray.origin;
        let t_center = l.dot(ray.direction);
        if t_center <= 0f64 {
            None
        } else {
            let d_sq = l.magnitude2() - t_center * t_center;
            let r_sq = self.radius * self.radius; // could cache?
            if d_sq > r_sq {
                None
            } else {
                let t_distance = (r_sq - d_sq).sqrt();
                let t0 = t_center - t_distance;
                let t1 = t_center + t_distance;
                let mut t = if t0 <= 0f64 { t1 } else { t0 };
                // TODO: pbrt recommends not twiddling t, but instead providing a precision value in the result that callers can use to twiddle the result as they want.
                t -= 1e-10;
                let location = ray.at(t);

                // pbrt pg. 119
                // Make sure we transform into object space!
                let mut phi = (location.y - self.center.y).atan2(location.x - self.center.x);
                if phi < 0f64 {
                    phi += 2f64 * PI;
                }
                let theta = ((location.z - self.center.z) / self.radius).clamp(-1f64, 1f64).acos();

                Some(Intersection {
                    distance: t,
                    location,
                    normal: (location - self.center).as_unit_vector(),
                    uv: (phi / (2f64 * PI), theta / PI),
                    object: self,
                })
            }
        }
    }

    fn material(&self) -> Rc<Material> { Rc::clone(&self.material) }
}
