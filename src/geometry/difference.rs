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

const EPSILON: f64 = 1e-10f64;

fn flip_normal(i: Intersection) -> Intersection {
    Intersection {
        distance: i.distance,
        location: i.location,
        normal: -i.normal,
        uv: i.uv,
    }
}

fn bump_distance(intersection: Option<Intersection>, distance: f64, direction: &Vec3) ->Option<Intersection> {
    match intersection {
        Some(i) => Some(Intersection {
            distance: i.distance + distance,
            // TODO: Ugh, Intersections really shouldn't be caching this.
            location: i.location + (*direction) * distance,
            normal: i.normal,
            uv: i.uv,
        }),
        None => None,
    }
}

impl Geometry for Difference {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        match self.lhs.intersect(ray) {
            None => None,
            Some(lhs_intersection) => {
                match self.rhs.intersect(ray) {
                    None => Some(lhs_intersection),
                    Some(rhs_intersection) => {
                        let inside_lhs = lhs_intersection.normal.dot(&-ray.direction) < 0f64;
                        let inside_rhs = rhs_intersection.normal.dot(&-ray.direction) < 0f64;
                        if inside_lhs && inside_rhs {
                            if lhs_intersection.distance < rhs_intersection.distance {
                                None
                            } else {
                                Some(flip_normal(rhs_intersection))
                            }
                        } else if inside_lhs {
                            if lhs_intersection.distance < rhs_intersection.distance {
                                Some(lhs_intersection)
                            } else {
                                Some(flip_normal(rhs_intersection))
                            }
                        } else if inside_rhs {
                            self.march_out_of_rhs_and_recurse(ray, rhs_intersection)
                        } else {
                            if lhs_intersection.distance < rhs_intersection.distance {
                                Some(lhs_intersection)
                            } else {
                                self.march_out_of_rhs_and_recurse(ray, rhs_intersection)
                            }
                        }
                    }
                }
            }
        }
    }

    fn bound(&self) -> BoundingBox {
        BoundingBox::union(&self.lhs.bound(), &self.rhs.bound())
    }
}

impl Difference {
    fn march_out_of_rhs_and_recurse(&self, ray: &Ray, rhs_intersection: Intersection) -> Option<Intersection> {
        let march_distance = rhs_intersection.distance + EPSILON;
        let marched_ray = Ray {
            origin: ray.at(march_distance),
            direction: ray.direction,
        };
        match self.lhs.intersect(&marched_ray) {
            Some(far_lhs_intersection) => {
                let inside_far_lhs = far_lhs_intersection.normal.dot(&-ray.direction) < 0f64;
                if inside_far_lhs {
                    Some(flip_normal(rhs_intersection))
                } else {
                    bump_distance(self.intersect(&marched_ray), march_distance, &ray.direction)
                }
            },
            None => bump_distance(self.intersect(&marched_ray), march_distance, &ray.direction)
        }
    }
}
