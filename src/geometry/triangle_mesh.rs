use std::rc::Rc;
use core::*;
use math::*;
use geometry::Geometry;
use kd_tree::KdTree;
use bounding_box::{ Bounded, BoundingBox };

pub type TriangleIndices = (usize, usize, usize);

#[derive(Debug)]
pub enum Smoothing {
    None,
    Implicit,
    Explicit(Vec<Normal>),
}

#[derive(Debug)]
pub struct TriangleMesh {
    triangles: KdTree<Triangle>,
    positions: Rc<Vec<Point>>,
    normals: Option<Vec<Normal>>,
    uvs: Option<Vec<Uv>>,
    closed: bool,
}

#[derive(Debug)]
struct Triangle {
    all_positions: Rc<Vec<Point>>,
    indices: TriangleIndices,
}

impl Bounded for Triangle {
    fn bound(&self) -> BoundingBox {
        BoundingBox::empty()
            .with_point(&self.all_positions[self.indices.0])
            .with_point(&self.all_positions[self.indices.1])
            .with_point(&self.all_positions[self.indices.2])
    }
}

impl TriangleMesh {
    // FYI, the "front" is when the vertices are in counterclockwise order, following OpenGL.
    pub fn new(positions: Vec<Point>, smoothing: Smoothing, uvs: Option<Vec<Uv>>, indices: Vec<TriangleIndices>, closed: bool) -> TriangleMesh {
        let normals = match smoothing {
            Smoothing::Explicit(normals) => {
                assert_eq!(positions.len(), normals.len());
                Some(normals)
            },
            Smoothing::Implicit => Some(TriangleMesh::compute_implicit_normals(&positions, &indices)),
            Smoothing::None => None,
        };

        if uvs.is_some() {
            assert_eq!(positions.len(), uvs.as_ref().unwrap().len());
        }

        let rc_positions = Rc::new(positions);

        let triangles = KdTree::from(indices
            .into_iter()
            .map(|indices| Triangle {
                all_positions: Rc::clone(&rc_positions),
                indices,
            })
            .collect());

        // TODO: Also check that the coordinates are in-bounds.
        TriangleMesh { positions: rc_positions, triangles, normals, uvs, closed }
    }

    fn compute_implicit_normals(positions: &Vec<Point>, indices: &Vec<TriangleIndices>) -> Vec<Normal> {
        let mut normals = vec![Normal::uniform(0f64); positions.len()];
        for &(i1, i2, i3) in indices {
            let normal = (positions[i3] - positions[i1]).cross(positions[i2] - positions[i1]).as_normal();
            normals[i1] = normals[i1] + normal;
            normals[i2] = normals[i2] + normal;
            normals[i3] = normals[i3] + normal;
        }
        normals.iter().map(|n| n.as_normalized()).collect()
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

        // prbt pg. 143
        // pbrt uses a different method to compute the normal, but it does use e2 x e1 in a special case
        // and refers to it as the normal, so we use that here.
        let mut normal = match self.normals {
            Some(ref normals) => {
                let b0 = 1f64 - b2 - b1;
                let (n0, n1, n2) = (normals[i1], normals[i2], normals[i3]);
                n0 * b0 + n1 * b1 + n2 * b2
            },
            None => e2.cross(e1).as_normalized().as_normal(),
        };

        // Flip normal if the mesh isn't closed and we hit the back
        if !self.closed && normal.dot(&ray.direction) > 0f64 {
            normal = -normal;
        }

        Some(Intersection {
            distance: t,
            location: ray.at(t),
            normal,
            uv: (0f64, 0f64),
        })
    }
}

impl Geometry for TriangleMesh {
    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        let mut closest: Option<Intersection> = None;

        for triangle in self.triangles.intersects(ray) {
            match self.intersect_triplet(&triangle.indices, ray) {
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
}

impl Bounded for TriangleMesh {
    fn bound(&self) -> BoundingBox {
        let mut bb = BoundingBox::empty();

        for ref p in self.positions.as_ref() {
            bb = bb.with_point(p);
        }

        bb
    }
}
