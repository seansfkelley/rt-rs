use std::f64::consts::PI;
use std::rc::Rc;

use core::*;
use math::*;
use material::Material;

#[derive(Debug)]
pub struct Sphere {
    radius: f64,
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

// pbrt pg. 118
fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let d = b * b - 4f64 * a * c;
    if d <= 0f64 {
        None
    } else {
        let sqrt_d = d.sqrt();
        let q = -0.5f64 * (b + (if b < 0f64 { -sqrt_d } else { sqrt_d }));
        let (t0, t1) = (q / a, c / q);
        if t0 > t1 {
            Some((t1, t0))
        } else {
            Some((t0, t1))
        }
    }
}

impl SceneObject for Sphere {
    fn intersect(&self, world_ray: &Ray) -> Option<Hit> {
        let object_ray = world_ray.transform(&self.transform);

        let (a, b, c) = (
            object_ray.direction.magnitude2(),
            2f64 * (object_ray.direction.dot(&object_ray.origin)),
            object_ray.origin.dot(&object_ray.origin) - self.radius * self.radius
        );

        let exit = match quadratic(a, b, c) {
            Some((t0, t1)) => {
                if t1 < 0f64 {
                    None
                } else if t0 < 0f64 {
                    Some(self.get_intersection(t1, world_ray, &object_ray))
                } else {
                    Some(self.get_intersection(t0, world_ray, &object_ray))
                }
            },
            None => None,
        };

        match exit {
            Some(intersection) => {
                Some(Hit {
                    enter: Some(intersection),
                    exit: Intersection {
                        distance: -100f64,
                        location: Point::new(0f64, 0f64, 0f64),
                        normal: Normal::new(0f64, 0f64, 0f64),
                        uv: (0f64, 0f64),
                    },
                    object: self,
                    debug: false
                })
            },
            None => None,
        }
    }

    fn material(&self) -> Rc<Material> { Rc::clone(&self.material) }
}
