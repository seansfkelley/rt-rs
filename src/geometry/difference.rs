use std::rc::Rc;

use core::*;
use math::*;
use geometry::Geometry;

#[derive(Debug)]
pub struct Difference {
    lhs: Rc<Geometry>,
    rhs: Rc<Geometry>,
}

impl Difference {
    pub fn new(lhs: Rc<Geometry>, rhs: Rc<Geometry>) -> Difference {
        Difference { lhs, rhs }
    }
}

fn flip_normal(i: Intersection) -> Intersection {
    Intersection {
        distance: i.distance,
        location: i.location,
        normal: -i.normal,
        uv: i.uv,
    }
}

impl Geometry for Difference {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self.lhs.intersect(ray) {
            Some(lhs_intersection) => {
                match self.rhs.intersect(ray) {
                    Some(rhs_intersection) => {
                        let entering_lhs = lhs_intersection.normal.dot(&-ray.direction) > 0f64;
                        let entering_rhs = rhs_intersection.normal.dot(&-ray.direction) > 0f64;
                        if entering_lhs && entering_rhs {
                            if lhs_intersection.distance < rhs_intersection.distance {
                                Some(lhs_intersection)
                            } else {
                                self.intersect(&Ray {
                                    origin: ray.at(lhs_intersection),
                                    direction: ray.direction,
                                })
                            }
                        } else if entering_lhs {

                        } else if entering_rhs {

                        } else {
                            Some(flip_normal(rhs_intersection))
                        }
                    }
                    None => Some(lhs_intersection)
                }
            }
            None => None
        }
    }
}
