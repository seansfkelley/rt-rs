use std::sync::Arc;
use std::f64;

use core::*;
use math::*;

#[derive(Debug)]
pub struct Difference {
    lhs: Arc<Geometry>,
    rhs: Arc<Geometry>,
}

impl Difference {
    pub fn new(lhs: Arc<Geometry>, rhs: Arc<Geometry>) -> Difference {
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
        material: i.material,
    }
}

impl Geometry for Difference {
    fn bound(&self) -> BoundingBox {
        BoundingBox::union(&self.lhs.bound(), &self.rhs.bound())
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // We have to remove the outer limit on rays because we also use them to determine
        // if we're inside geometries. If we're inside but the limit doesn't let us hit the
        // surface, we assume we don't collide at all and weird things happen!
        match self.internal_intersect(ray.clone().with_max(f64::INFINITY)) {
            Some(intersection) => {
                if intersection.distance > ray.t_max {
                    None
                } else {
                    Some(intersection)
                }
            },
            None => None,
        }
    }
}

impl Difference {
    fn internal_intersect(&self, ray: Ray) -> Option<Intersection> {
        match self.lhs.intersect(&ray) {
            None => None,
            Some(lhs_intersection) => {
                match self.rhs.intersect(&ray) {
                    None => Some(lhs_intersection),
                    Some(rhs_intersection) => self.internal_intersect_nontrivial(ray, lhs_intersection, rhs_intersection),
                }
            }
        }
    }

    fn internal_intersect_nontrivial(&self, ray: Ray, lhs_intersection: Intersection, rhs_intersection: Intersection) -> Option<Intersection> {
        let inside_lhs = lhs_intersection.normal.dot(&ray.direction) > 0f64;
        let inside_rhs = rhs_intersection.normal.dot(&ray.direction) > 0f64;
        if inside_lhs && inside_rhs {
            if lhs_intersection.distance < rhs_intersection.distance {
                let d = lhs_intersection.distance + EPSILON;
                self.internal_intersect(ray.with_min(d))
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
            self.internal_intersect(ray.with_min(d))
        } else {
            if lhs_intersection.distance < rhs_intersection.distance {
                Some(lhs_intersection)
            } else {
                let d = rhs_intersection.distance + EPSILON;
                self.internal_intersect(ray.with_min(d))
            }
        }
    }
}
