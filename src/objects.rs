use vector::Vec3;
use color::Color;

pub struct Material {
    diffuse: Color,
    specular: Color,
    specular_exponent: f64,
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
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

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub distance: f64,
    pub normal: Vec3,
    pub material: Material,
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Option<Intersection>;
}

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Sphere {
        Sphere { center, radius, material }
    }
}

impl Intersectable for Sphere {
    // TODO: Understand what I actually wrote here.
    // from http://www.scratchapixel.com/lessons/3d-basic-rendering/minimal-ray-tracer-rendering-simple-shapes/ray-sphere-intersection
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
                let t = if t0 <= 0f64 { t0 } else { t1 };

                Some(Intersection {
                    distance: t,
                    normal: ((ray.direction * t + ray.origin) - self.center).as_unit_vector(),
                    material: self.material
                })
            }
        }
    }
}
