use std::rc::Rc;

use core::*;
use material::Material;

pub struct Difference {
    lhs: Rc<SceneObject>,
    rhs: Rc<SceneObject>,
}

impl Difference {
    pub fn new(lhs: Rc<SceneObject>, rhs: Rc<SceneObject>) -> Difference {
        Difference { lhs, rhs }
    }
}

impl SceneObject for Difference {
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