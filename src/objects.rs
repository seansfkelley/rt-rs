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

pub struct Intersection {
    pub distance: f64,
    pub location: Vec3,
    pub normal: Vec3,
    pub uv: (f64, f64),
}

pub struct Hit<'a> {
    pub enter: Option<Intersection>,
    pub exit: Intersection,
    pub object: &'a (SceneObject + 'a),
}

pub trait SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
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

    fn get_intersection(&self, t: f64, ray: &Ray) -> Intersection {
        let location = ray.at(t);

        // pbrt pg. 119
        // Make sure we transform into object space!
        let mut phi = (location.y - self.center.y).atan2(location.x - self.center.x);
        if phi < 0f64 {
            phi += 2f64 * PI;
        }
        let theta = ((location.z - self.center.z) / self.radius).clamp(-1f64, 1f64).acos();

        Intersection {
            distance: t,
            location,
            normal: (location - self.center).as_unit_vector(),
            uv: (phi / (2f64 * PI), theta / PI),
        }
    }
}

impl SceneObject for Sphere {
    // TODO: Verify this implementation against pbrt.
    // TODO: Should transform ray into world space first so the rest of the math is easy.
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
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
                if t0 <= 0f64 {
                    Some(Hit {
                        enter: None,
                        exit: self.get_intersection(t1, ray),
                        object: self
                    })
                } else {
                    Some(Hit {
                        enter: Some(self.get_intersection(t0, ray)),
                        exit: self.get_intersection(t1, ray),
                        object: self
                    })
                }
            }
        }
    }

    fn material(&self) -> Rc<Material> { Rc::clone(&self.material) }
}

pub struct SubtractedSceneObject {
    lhs: Rc<SceneObject>,
    rhs: Rc<SceneObject>,
}

impl SceneObject for SubtractedSceneObject {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let lhs_hit_option = self.lhs.intersect(ray);

        match lhs_hit_option {
            Some(lhs_hit) => {
                let rhs_hit_option = self.rhs.intersect(ray);
                match rhs_hit_option {
                    Some(rhs_hit) => {
                        let lhs_enter = lhs_hit.enter.as_ref().map(|enter| enter.distance).unwrap_or(0f64);
                        let rhs_enter = rhs_hit.enter.map(|enter| enter.distance).unwrap_or(0f64);
                        let lhs_exit = lhs_hit.exit.distance;
                        let rhs_exit = rhs_hit.exit.distance;
                        if lhs_enter < rhs_enter || lhs_enter > rhs_exit {
                            Some(lhs_hit)
                        } else {
                            // lhs_enter is inside rhs
                            if rhs_exit < lhs_exit {
                                // Exists from rhs first
                                Some(Hit {
                                    enter: Some(Intersection {
                                        distance: rhs_hit.exit.distance,
                                        location: rhs_hit.exit.location,
                                        normal: -rhs_hit.exit.normal,
                                        uv: rhs_hit.exit.uv,
                                    }),
                                    exit: lhs_hit.exit,
                                    object: lhs_hit.object,
                                })
                            } else {
                                None
                            }
                        }
                    },
                    None => Some(lhs_hit),
                }
            },
            None => None,
        }
    }

    fn material(&self) -> Rc<Material> {
        self.lhs.material()
    }
}

pub fn subtract_scene_objects(lhs: Rc<SceneObject>, rhs: Rc<SceneObject>) -> SubtractedSceneObject {
    SubtractedSceneObject { lhs, rhs }
}
