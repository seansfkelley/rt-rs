use std::f64::consts::PI;
use std::rc::Rc;

use core::*;
use math::*;
use material::Material;

#[derive(Debug)]
pub struct Sphere {
    radius: f64,
    // TODO: Should only have a material and we should create a
    // `TransformedObject` that is a scene object and a transform
    transform: Transform,
    material: Rc<Material>,
}

impl Sphere {
    pub fn new(radius: f64, transform: Transform, material: &Rc<Material>) -> Sphere {
        Sphere {
            radius,
            transform,
            material: Rc::clone(material),
        }
    }

    fn get_intersection(&self, t: f64, world_ray: &Ray, object_ray: &Ray) -> Intersection {
        let object_location = object_ray.at(t);
        let world_location = object_location.transform(&self.transform);

        // pbrt pg. 119
        let mut phi = object_location.y.atan2(object_location.x);
        if phi < 0f64 {
            phi += 2f64 * PI;
        }
        let theta = object_location.z.acos();

        Intersection {
            distance: world_location.to_vector().dot(&world_ray.direction),
            location: world_location,
            normal: object_location.transform(&self.transform).to_vector().to_normal().as_normalized(),
            uv: (phi / (2f64 * PI), theta / PI),
        }
    }
}

impl SceneObject for Sphere {
    // TODO: Verify this implementation against pbrt.
    fn intersect(&self, world_ray: &Ray) -> Option<Hit> {
        let object_ray = world_ray.transform(&self.transform);
        let l = -object_ray.origin.to_vector();
        let t_center = l.dot(&object_ray.direction);

        if t_center + self.radius <= 0f64 {
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
