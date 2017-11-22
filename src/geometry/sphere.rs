use std::f64::consts::PI;

use core::*;
use geometry::Geometry;
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
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let (a, b, c) = (
            ray.direction.magnitude2(),
            2f64 * (ray.direction.dot(&ray.origin)),
            ray.origin.dot(&ray.origin) - self.radius * self.radius
        );

        match quadratic(a, b, c) {
            Some((t0, t1)) => {
                if t1 < 0f64 {
                    None
                } else if t0 < 0f64 {
                    Some(self.get_intersection(t1, &ray))
                } else {
                    Some(self.get_intersection(t0, &ray))
                }
            }
            None => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const STRAIGHT_RAY: Ray = Ray {
        origin: Point { x: 0f64, y: 0f64, z: 5f64 },
        direction: Vec3 { x: 0f64, y: 0f64, z: -1f64 }
    };

    const OFFSET_RAY: Ray = Ray {
        origin: Point { x: 5f64, y: 0f64, z: 5f64 },
        direction: Vec3 { x: 0f64, y: 0f64, z: -1f64 }
    };

    #[test]
    fn it_should_intersect() {
        let sphere = Sphere::new(1f64);
        assert!(sphere.intersect(&STRAIGHT_RAY).is_some());
        assert!(sphere.intersect(&OFFSET_RAY).is_none());
    }

    // #[test]
    // fn it_should_intersect_translations() {
    //     let sphere = Sphere::new(1f64, Mat4::create_translation(Vec3::new(5f64, 0f64, 0f64)), BLANK_MATERIAL);
    //     assert!(sphere.intersect(&STRAIGHT_RAY).is_none());
    //     assert!(sphere.intersect(&OFFSET_RAY).is_some());
    // }
}
