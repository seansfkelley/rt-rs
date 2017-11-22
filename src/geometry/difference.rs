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

fn bump_distance(i: Intersection, distance: f64, direction: Vec3) -> Intersection {
    Intersection {
        distance: i.distance + distance,
        // TODO: Ugh, Intersections really shouldn't be caching this.
        location: i.location + direction * distance,
        normal: i.normal,
        uv: i.uv,
    }
}

impl Geometry for Difference {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // This function is a doozy. Let me try to explain.
        match self.lhs.intersect(ray) {
            // If we don't even hit the positive side, trivially, we don't intersect the difference.
            None => None,
            Some(lhs_intersection) => {
                // If we hit the positive side, fire a ray from the intersection point and see if we're inside the negative.
                match self.rhs.intersect(&Ray {
                    origin: lhs_intersection.location,
                    direction: ray.direction,
                }) {
                    // If we don't hit the negative side at all, we can't be inside it, so regardless of where we intersected
                    // the positive side that's what we use.
                    None => Some(lhs_intersection),
                    Some(forward_rhs_intersection) => {
                        let inside_rhs = forward_rhs_intersection.normal.dot(&-ray.direction) < 0f64;
                        // If we hit the negative side, but not because we were inside it, then the negative side is not
                        // relevant to this location in space and we just return the positive intersection.
                        if !inside_rhs {
                            Some(lhs_intersection)
                        } else {
                            // TODO: if we're starting outside the positive side this should work, not sure about starting inside

                            // If we're inside the negative side, follow the ray until we get out and see if that location is
                            // still inside the positive side.
                            match self.lhs.intersect(&Ray {
                                origin: forward_rhs_intersection.location,
                                direction: ray.direction,
                            }) {
                                // If we don't hit the positive side at all, we can't be inside it, so this zone of the negative
                                // shape contains the positive shape entirely and we ahve no intersection.
                                None => None,
                                Some(forward_lhs_intersection) => {
                                    let inside_lhs = forward_lhs_intersection.normal.dot(&-ray.direction) < 0f64;
                                    // If we're inside, we have a good intersection. Flip the normal, because this point
                                    // is actually at the boundary of the negative and positive and we're on the surface
                                    // of the _difference_ shape (not the positive shape), i.e., point outside.
                                    if inside_lhs {
                                        Some(flip_normal(forward_lhs_intersection))
                                    } else {
                                        // This is a case that can only happen with non-convex shapes, I'm pretty sure:
                                        // we left the shape and this new ray is entering it again. Restart the process,
                                        // since the negative shape has completely obliterated the positive shape in this
                                        // zone and we need to try again.
                                        match self.intersect(&Ray {
                                            origin: forward_rhs_intersection.location,
                                            direction: ray.direction,
                                        }) {
                                            // Since we moved the origin around, add the necessary distance to it so the
                                            // original caller gets a sane value.
                                            Some(delegate_intersection) => Some(bump_distance(delegate_intersection, forward_rhs_intersection.distance, ray.direction)),
                                            None => None
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}
