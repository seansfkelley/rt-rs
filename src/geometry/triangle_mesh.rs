use core::*;
use math::*;
use geometry::Geometry;

pub type TriangleIndices = (usize, usize, usize);

#[derive(Debug)]
pub struct TriangleMesh {
    positions: Vec<Point>,
    normals: Vec<Normal>,
    uvs: Vec<Uv>,
    indices: Vec<TriangleIndices>,
}

impl TriangleMesh {
    // FYI, the "front" is when the vertices are in counterclockwise order, following OpenGL.
    pub fn new(positions: Vec<Point>, normals: Vec<Normal>, uvs: Vec<Uv>, indices: Vec<TriangleIndices>) -> TriangleMesh {
        // TODO: When we actually do UVs and given normals, do this.
        // assert_eq!(positions.len(), normals.len());
        // assert_eq!(positions.len(), uvs.len());
        // TODO: Also check that the coordinates are in-bounds.
        TriangleMesh { positions, normals, uvs, indices }
    }

    fn intersect_triplet(&self, triplet: &TriangleIndices, ray: &Ray) -> Option<Intersection> {
        // pbrt pg. 141
        let &(i1, i2, i3) = triplet;
        let (p1, p2, p3) = (self.positions[i1], self.positions[i2], self.positions[i3]);
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let s1 = ray.direction.cross(e2);
        let divisor = s1.dot(&e1);
        if divisor == 0f64 {
            return None;
        }

        let d = ray.origin - p1;
        let b1 = d.dot(&s1) / divisor;
        if b1 < 0f64 || b1 > 1f64 {
            return None;
        }

        let s2 = d.cross(e1);
        let b2 = ray.direction.dot(&s2) / divisor;
        if b2 < 0f64 || b1 + b2 > 1f64 {
            return None;
        }

        let t = e2.dot(&s2) / divisor;
        if t < 0f64 {
            return None;
        }

        Some(Intersection {
            distance: t,
            location: ray.at(t),
            // prbt pg. 143
            // pbrt uses a different method to compute the normal, but it does use e2 x e1 in a special case
            // and refers to it as the normal, so we use that here.
            normal: e2.cross(e1).as_normalized().as_normal(),
            uv: (0f64, 0f64),
        })
    }
}

impl Geometry for TriangleMesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest: Option<Intersection> = None;

        for triplet in &self.indices {
            match self.intersect_triplet(&triplet, ray) {
                Some(intersection) => {
                    closest = match closest {
                        Some(closest_intersection) => {
                            if intersection.distance < closest_intersection.distance {
                                Some(intersection)
                            } else {
                                Some(closest_intersection)
                            }
                        },
                        None => Some(intersection),
                    };
                }
                None => {}
            }
        }

        closest
    }

    fn bound(&self) -> BoundingBox {
        let mut bb = BoundingBox::empty();

        for ref p in &self.positions {
            bb = bb.with_point(p);
        }

        bb
    }
}
