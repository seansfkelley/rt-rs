use std::f64::consts::PI;
use std::rc::Rc;

use core::*;
use transform::Mat4;
use material::Material;
use geometry::Geometry;
use util::Clamp;

#[derive(Debug)]
pub struct Sphere {
    radius: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Sphere {
        Sphere { radius }
    }

    fn get_intersection(&self, t: f64, ray: &Ray) -> Intersection {
        let location = ray.at(t);

        // pbrt pg. 119
        // Make sure we transform into object space!
        let mut phi = location.y.atan2(location.x);
        if phi < 0f64 {
            phi += 2f64 * PI;
        }
        let theta = (location.z / self.radius).clamp(-1f64, 1f64).acos();

        Intersection {
            distance: t,
            location,
            normal: location.as_unit_vector(),
            uv: (phi / (2f64 * PI), theta / PI),
        }
    }
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let l = ray.origin;
        let t_center = l.dot(ray.direction);
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
                let exit = self.get_intersection(t1, ray);
                if t0 <= 0f64 {
                    Some(Hit {
                        enter: None,
                        exit,
                        debug: false,
                    })
                } else {
                    Some(Hit {
                        enter: Some(self.get_intersection(t0, ray)),
                        exit,
                        debug: false,
                    })
                }
            }
        }
    }
}
