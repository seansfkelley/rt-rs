use std::f64::consts::PI;
use std::rc::Rc;

use core::*;
use material::Material;
use geometry::Geometry;
use util::Clamp;
use math::*;

#[derive(Debug)]
pub struct Sphere {
    radius: f64,
}

impl Sphere {
    pub fn new(radius: f64) -> Sphere {
        Sphere {
            radius,
        }
    }

    fn get_intersection(&self, t: f64, ray: &Ray) -> Intersection {
        let intersection_point = ray.at(t);

        // pbrt pg. 119
        let mut phi = intersection_point.y.atan2(intersection_point.x);
        if phi < 0f64 {
            phi += 2f64 * PI;
        }
        let theta = (intersection_point.z / self.radius).acos();

        Intersection {
            distance: t,
            location: intersection_point,
            normal: intersection_point.as_normal().as_normalized(),
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

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let (a, b, c) = (
            ray.direction.magnitude2(),
            2f64 * (ray.direction.dot(&ray.origin)),
            ray.origin.dot(&ray.origin) - self.radius * self.radius
        );

        let enter = match quadratic(a, b, c) {
            Some((t0, t1)) => {
                if t1 < 0f64 {
                    None
                } else if t0 < 0f64 {
                    Some(self.get_intersection(t1, &ray))
                } else {
                    Some(self.get_intersection(t0, &ray))
                }
            },
            None => None,
        };

        match enter {
            Some(intersection) => {
                Some(Hit {
                    enter: Some(intersection),
                    exit: Intersection {
                        distance: -100f64,
                        location: Point::new(0f64, 0f64, 0f64),
                        normal: Normal::new(0f64, 0f64, 0f64),
                        uv: (0f64, 0f64),
                    },
                    debug: false
                })
            },
            None => None,
        }
    }
}
