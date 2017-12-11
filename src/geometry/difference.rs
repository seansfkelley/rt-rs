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

// TODO: Should we also move max_t?
fn advance_ray(ray: &Ray, t: f64) -> Ray {
    Ray {
        origin: ray.origin,
        direction: ray.direction,
        t_min: ray.t_min + t,
        t_max: ray.t_max + t,
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

fn advance_intersection(intersection: Option<Intersection>, distance: f64) ->Option<Intersection> {
    match intersection {
        Some(i) => Some(Intersection {
            distance: i.distance + distance,
            location: i.location,
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
                    Some(rhs_intersection) => self.non_trivial_intersect(ray, lhs_intersection, rhs_intersection),
                }
            }
        }
    }

    fn bound(&self) -> BoundingBox {
        BoundingBox::union(&self.lhs.bound(), &self.rhs.bound())
    }
}

impl Difference {
    fn non_trivial_intersect(&self, ray: &Ray, lhs_intersection: Intersection, rhs_intersection: Intersection) -> Option<Intersection> {
        let inside_lhs = lhs_intersection.normal.dot(&-ray.direction) < 0f64;
        let inside_rhs = rhs_intersection.normal.dot(&-ray.direction) < 0f64;
        if inside_lhs && inside_rhs {
            if lhs_intersection.distance < rhs_intersection.distance {
                let d = lhs_intersection.distance + EPSILON;
                advance_intersection(self.intersect(&advance_ray(ray, d)), d)
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
            // TODO: We could optimize this to only recompute the near intersection, but I'm lazy
            // and also it's possible for the two shapes to be perfectly colocated such that the
            // marched ray would e.g. miss both, but then you reused one of the old intersections
            // erroneously.
            let d = if lhs_intersection.distance < rhs_intersection.distance {
                lhs_intersection.distance
            } else {
                rhs_intersection.distance
            } + EPSILON;
            advance_intersection(self.intersect(&advance_ray(ray, d)), d)
        } else {
            if lhs_intersection.distance < rhs_intersection.distance {
                Some(lhs_intersection)
            } else {
                let d = rhs_intersection.distance + EPSILON;
                advance_intersection(self.intersect(&advance_ray(ray, d)), d)
            }
        }
    }
}
