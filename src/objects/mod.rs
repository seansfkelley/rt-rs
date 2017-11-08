use vector::Vec3;
use color::Color;
use material::Material;
use util::Clamp;
use transform::Mat4;
use std::f64::consts::PI;
use std::rc::Rc;

pub mod sphere;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        direction.assert_normalized();
        Ray { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Vec3 {
        self.origin + self.direction * distance
    }

    pub fn transform(&self, transform: Mat4) -> Ray {
        let origin = transform * self.origin;
        let direction = (transform * self.direction).as_unit_vector();
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
    pub debug: bool,
}

impl<'a> Hit<'a> {
    pub fn debug(self, debug: bool) -> Hit<'a> {
        Hit {
            enter: self.enter,
            exit: self.exit,
            object: self.object,
            debug,
        }
    }
}

pub trait SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    fn material(&self) -> Rc<Material>;
}

#[derive(Debug)]
pub struct Cube {
    transform: Mat4,
    material: Rc<Material>,
}

impl SceneObject for Cube {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        None
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
                                    debug: false,
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
