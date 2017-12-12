use std::rc::Rc;
use core::*;
use math::*;

pub type TriangleIndices = (usize, usize, usize);

#[derive(Debug)]
pub enum Smoothing {
    None,
    Implicit,
    Explicit(Vec<Normal>),
}

#[derive(Debug)]
struct TriangleMeshData {
    positions: Vec<Point>,
    normals: Option<Vec<Normal>>,
    uvs: Option<Vec<Uv>>,
    closed: bool,
}

#[derive(Debug)]
struct Triangle {
    mesh: Rc<TriangleMeshData>,
    indices: TriangleIndices,
}

impl Geometry for Triangle {
    fn bound(&self) -> BoundingBox {
        BoundingBox::empty()
            .with_point(&self.mesh.positions[self.indices.0])
            .with_point(&self.mesh.positions[self.indices.1])
            .with_point(&self.mesh.positions[self.indices.2])
    }

    fn intersect(&self, ray: &Ray) -> Option<Intersection> {
        // pbrt pg. 141
        let (i1, i2, i3) = self.indices;
        let (p1, p2, p3) = (self.mesh.positions[i1], self.mesh.positions[i2], self.mesh.positions[i3]);
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
        if t < ray.t_min || t > ray.t_max {
            return None;
        }

        // prbt pg. 143
        let mut normal = match self.mesh.normals {
            Some(ref normals) => {
                let b0 = 1f64 - b2 - b1;
                let (n0, n1, n2) = (normals[i1], normals[i2], normals[i3]);
                n0 * b0 + n1 * b1 + n2 * b2
            },
            None => e2.cross(e1).as_normalized().as_normal(),
        };

        // Flip normal if the mesh isn't closed and we hit the back
        if !self.mesh.closed && normal.dot(&ray.direction) > 0f64 {
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

pub type TriangleMesh = KdTree<Triangle>;

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

        let mesh = Rc::new(TriangleMeshData { positions, normals, uvs, closed });

        KdTree::from(indices
            .into_iter()
            .map(|indices| Triangle {
                mesh: Rc::clone(&mesh),
                indices,
            })
            .collect())
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
}

// #[cfg(test)]
// mod tests {
//     use super::*;

//     lazy_static! {
//         static ref SINGLE_TRIANGLE: TriangleMeshData = TriangleMeshData::new(
//             vec![Point::new(-1f64, -1f64, 0f64), Point::new(1f64, -1f64, 0f64), Point::new(0f64, 1f64, 0f64)],
//             Smoothing::None,
//             None,
//             vec![(0, 1, 2)],
//             true,
//         );
//     }

//     #[test]
//     fn it_should_intersect_a_half_infinite_ray() {
//         let r = Ray::half_infinite(Point::new(0f64, 0f64, -3f64), Vec3::Z_AXIS);
//         let i = SINGLE_TRIANGLE.intersect(&r);
//         assert!(i.is_some());
//         assert_eq!(i.unwrap().distance, 3f64);
//     }

//     #[test]
//     fn it_should_intersect_a_finite_ray() {
//         let r = Ray::finite(Point::new(0f64, 0f64, -3f64), Vec3::Z_AXIS, 0f64, 6f64);
//         let i = SINGLE_TRIANGLE.intersect(&r);
//         assert!(i.is_some());
//         assert_eq!(i.unwrap().distance, 3f64);
//     }

//     #[test]
//     fn it_should_not_intersect_a_half_infinite_ray() {
//         let r = Ray::half_infinite(Point::new(5f64, 0f64, -5f64), Vec3::Z_AXIS);
//         assert!(SINGLE_TRIANGLE.intersect(&r).is_none());
//     }

//     #[test]
//     fn it_should_not_intersect_a_finite_ray() {
//         let r = Ray::finite(Point::new(0f64, 0f64, -5f64), Vec3::Z_AXIS, 0f64, 1f64);
//         assert!(SINGLE_TRIANGLE.intersect(&r).is_none());
//     }
// }
