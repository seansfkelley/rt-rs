use vector::Vec3;
use color::Color;
use material::Material;

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

#[derive(Debug, Clone, Copy)]
pub struct Intersection {
    pub distance: f64,
    pub location: Vec3,
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
                let mut t = if t0 <= 0f64 { t1 } else { t0 };
                t -= 0.00000001f64;
                let location = ray.at(t);
                Some(Intersection {
                    distance: t,
                    location,
                    normal: (location - self.center).as_unit_vector(),
                    material: self.material
                })
            }
        }
    }
}