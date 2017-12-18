use std::sync::Arc;
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
pub struct Triangle {
    mesh: Arc<TriangleMeshData>,
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
        // Note that we renamed the indices to be consistent with the mathematical notation and so that
        // the indices were consistent between point, barycentric and normal names.
        let (i0, i1, i2) = self.indices;
        let (p0, p1, p2) = (self.mesh.positions[i0], self.mesh.positions[i1], self.mesh.positions[i2]);
        let e1 = p1 - p0;
        let e2 = p2 - p0;
        let s1 = ray.direction.cross(e2);
        let divisor = s1.dot(&e1);
        if divisor == 0f64 {
            return None;
        }

        let d = ray.origin - p0;
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

        let mut shading_normal = self.mesh.normals.as_ref().map(|normals| {
            let b0 = 1f64 - b2 - b1;
            let (n0, n1, n2) = (normals[i0], normals[i1], normals[i2]);
            n0 * b0 + n1 * b1 + n2 * b2
        });

        let mut normal = e2.cross(e1).into_normalized().into_normal();

        if !self.mesh.closed && normal.dot(&ray.direction) > 0f64 {
            normal = -normal;
            shading_normal = shading_normal.map(|n| -n);
        };

        Some(Intersection {
            distance: t,
            location: ray.at(t),
            normal,
            shading_normal,
            uv: (0f64, 0f64),
            material: None,
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
            }
            Smoothing::Implicit => Some(TriangleMesh::compute_implicit_normals(&positions, &indices)),
            Smoothing::None => None,
        };

        if uvs.is_some() {
            assert_eq!(positions.len(), uvs.as_ref().unwrap().len());
        }

        let mesh = Arc::new(TriangleMeshData { positions, normals, uvs, closed });

        KdTree::from(indices
            .into_iter()
            .map(|indices| Triangle {
                mesh: Arc::clone(&mesh),
                indices,
            })
            .collect())
    }

    fn compute_implicit_normals(positions: &Vec<Point>, indices: &Vec<TriangleIndices>) -> Vec<Normal> {
        let mut normals = vec![Normal::uniform(0f64); positions.len()];
        for &(i0, i1, i2) in indices {
            let normal = (positions[i2] - positions[i0]).cross(positions[i1] - positions[i0]).into_normal();
            normals[i0] = normals[i0] + normal;
            normals[i1] = normals[i1] + normal;
            normals[i2] = normals[i2] + normal;
        }
        normals.into_iter().map(|n| n.into_normalized()).collect()
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
