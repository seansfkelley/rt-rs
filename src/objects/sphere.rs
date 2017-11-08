use vector::Vec3;
use color::Color;
use material::Material;
use util::Clamp;
use transform::Mat4;
use std::f64::consts::PI;
use std::rc::Rc;
use super::{Ray, Hit, Intersection, SceneObject};

#[derive(Debug)]
pub struct Sphere {
    transform: Mat4,
    inverse_transform: Mat4,
    material: Rc<Material>,
}

const RADIUS: f64 = 1f64;

impl Sphere {
    pub fn new(transform: Mat4, material: Rc<Material>) -> Sphere {
        let inverse_transform = transform.invert().unwrap();
        Sphere {
            transform,
            material,
            inverse_transform,
        }
    }

    fn get_intersection(&self, t: f64, world_ray: &Ray, object_ray: &Ray) -> Intersection {
        let object_location = object_ray.at(t);
        let world_location = self.transform * object_location;

        // pbrt pg. 119
        let mut phi = object_location.y.atan2(object_location.x);
        if phi < 0f64 {
            phi += 2f64 * PI;
        }
        let theta = object_location.z.acos();

        Intersection {
            distance: world_location.dot(world_ray.direction),
            location: world_location,
            normal: (self.transform * object_location).as_unit_vector(),
            uv: (phi / (2f64 * PI), theta / PI),
        }
    }
}

impl SceneObject for Sphere {
    // TODO: Verify this implementation against pbrt.
    fn intersect(&self, world_ray: &Ray) -> Option<Hit> {
        let object_ray = world_ray.transform(self.inverse_transform);
        let l = -object_ray.origin;
        let t_center = l.dot(object_ray.direction);

        println!("object_ray: {:?}", object_ray);
        println!("l: {:?}", l);
        println!("t_center: {:?}", t_center);
        if t_center + RADIUS <= 0f64 {
            println!("none 1");
            None
        } else {
            let d_sq = l.magnitude2() - t_center * t_center;
            if d_sq > RADIUS {
                println!("none 2");
                None
            } else {
                let t_distance = (RADIUS - d_sq).sqrt();
                let t0 = t_center - t_distance;
                let t1 = t_center + t_distance;
                let exit = self.get_intersection(t1, world_ray, &object_ray);
                if t0 <= 0f64 {
                    Some(Hit {
                        enter: None,
                        exit,
                        object: self,
                        debug: false,
                    })
                } else {
                    Some(Hit {
                        enter: Some(self.get_intersection(t0, world_ray, &object_ray)),
                        exit,
                        object: self,
                        debug: false,
                    })
                }
            }
        }
    }

    fn material(&self) -> Rc<Material> { Rc::clone(&self.material) }
}
