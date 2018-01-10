use std::f64::consts::PI;
use core::*;
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
        const TWO_PI: f64 = PI * 2f64;

        let location = ray.at(t);
        let uv = sphere_uv_for_normalized_point(location / self.radius);
        let (phi, theta) = sphere_uv_to_polar(uv);
        // pbrt pg. 121, tweaked for different u/v formula (y/z swapped)
        let u_axis = Vec3::new(
            // The factor of TWO_PI means that the range [0, 1] for u will trace out the entire
            // latitutde line (corresponding to v) exactly. If it were normalized, it'd require
            // a range of [0, 2 * PI * radius-at-the-latitude] to do so.
            -TWO_PI * location.z,
            0f64,
            TWO_PI * location.x,
        );
        let v_axis = Vec3::new(
            // This has a factor of PI for the same reason u_axis has a factor of TWO_PI.
            // pbrt uses some clever equivalencies to compute the sin/cos here without referring to
            // phi, but whatever. This is more straightforward.
            PI * location.y * phi.cos(),
            -PI * self.radius * theta.sin(),
            PI * location.y * phi.sin(),
        );

        Intersection {
            distance: t,
            location,
            geometry: IntersectionGeometry::new(u_axis, v_axis),
            shading_geometry: None,
            uv,
            material: None,
        }
    }
}

// pbrt pg. 118
fn quadratic(a: f64, b: f64, c: f64) -> Option<(f64, f64)> {
    let d = b * b - 4f64 * a * c;
    if d < 0f64 {
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
    fn bound(&self) -> BoundingBox {
        BoundingBox {
            min: Point::uniform(-self.radius),
            max: Point::uniform(self.radius),
        }
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let (a, b, c) = (
            ray.direction.magnitude2(),
            2f64 * (ray.direction.dot(&ray.origin)),
            ray.origin.dot(&ray.origin) - self.radius * self.radius
        );

        match quadratic(a, b, c) {
            Some((t0, t1)) => {
                if t1 < ray.t_min || t0 > ray.t_max {
                    None
                } else if t0 < ray.t_min {
                    if t1 <= ray.t_max {
                        Some(self.get_intersection(t1, &ray))
                    } else {
                        None
                    }
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

    const UNIT_SPHERE: Sphere = Sphere { radius: 1f64 };

    #[test]
    fn it_should_intersect_a_half_infinite_ray_from_outside() {
        let r = Ray::half_infinite(Point::new(0f64, 0f64, -5f64), Vec3::Z_AXIS);
        let i = UNIT_SPHERE.intersect(&r);
        assert!(i.is_some());
        assert_eq!(i.unwrap().distance, 4f64);
    }

    #[test]
    fn it_should_intersect_a_finite_ray_from_outside() {
        let r = Ray::finite(Point::new(0f64, 0f64, -5f64), Vec3::Z_AXIS, 0f64, 5f64);
        let i = UNIT_SPHERE.intersect(&r);
        assert!(i.is_some());
        assert_eq!(i.unwrap().distance, 4f64);
    }

    #[test]
    fn it_should_intersect_a_finite_ray_from_inside() {
        let r = Ray::finite(Point::new(0f64, 0f64, 0f64), Vec3::Z_AXIS, 0f64, 5f64);
        let i = UNIT_SPHERE.intersect(&r);
        assert!(i.is_some());
        assert_eq!(i.unwrap().distance, 1f64);
    }

    #[test]
    fn it_should_not_intersect_a_half_infinite_ray_from_outside() {
        let r = Ray::half_infinite(Point::new(5f64, 0f64, -5f64), Vec3::Z_AXIS);
        assert!(UNIT_SPHERE.intersect(&r).is_none());
    }

    #[test]
    fn it_should_not_intersect_a_finite_ray_from_outside() {
        let r = Ray::finite(Point::new(0f64, 0f64, -5f64), Vec3::Z_AXIS, 0f64, 1f64);
        assert!(UNIT_SPHERE.intersect(&r).is_none());
    }

    #[test]
    fn it_should_not_intersect_a_finite_ray_from_inside() {
        let r = Ray::finite(Point::new(0f64, 0f64, 0f64), Vec3::Z_AXIS, 0f64, 0.5f64);
        assert!(UNIT_SPHERE.intersect(&r).is_none());
    }
}
