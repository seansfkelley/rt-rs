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
pub struct TriangleMeshData {
    pub positions: Vec<Point>,
    pub indices: Vec<TriangleIndices>,
    pub normals: Option<Vec<Normal>>,
    pub uvs: Option<Vec<Uv>>,
}

pub type TriangleMesh = VolumeKdTree<Triangle>;

impl TriangleMeshData {
    // FYI, the "front" is when the vertices are in counterclockwise order, following OpenGL.
    pub fn new(positions: Vec<Point>, smoothing: Smoothing, uvs: Option<Vec<Uv>>, indices: Vec<TriangleIndices>) -> TriangleMeshData {
        let normals = match smoothing {
            Smoothing::Explicit(normals) => {
                assert_eq!(positions.len(), normals.len());
                Some(normals)
            }
            Smoothing::Implicit => Some(TriangleMeshData::compute_implicit_normals(&positions, &indices)),
            Smoothing::None => None,
        };

        if uvs.is_some() {
            assert_eq!(positions.len(), uvs.as_ref().unwrap().len());
        }

        TriangleMeshData { positions, indices: indices.clone(), normals, uvs }
    }

    pub fn into_triangle_mesh(self) -> TriangleMesh {
        let mesh = Arc::new(self);

        KdTree::from(mesh.indices
            .iter()
            .map(|indices| Triangle {
                mesh: Arc::clone(&mesh),
                indices: *indices,
            })
            .collect())
    }

    fn compute_implicit_normals(positions: &Vec<Point>, indices: &Vec<TriangleIndices>) -> Vec<Normal> {
        let mut normals = vec![Normal::uniform(0f64); positions.len()];
        for &(i0, i1, i2) in indices {
            let normal = (positions[i1] - positions[i0]).cross(positions[i2] - positions[i0]).into_normal();
            normals[i0] = normals[i0] + normal;
            normals[i1] = normals[i1] + normal;
            normals[i2] = normals[i2] + normal;
        }
        normals.into_iter().map(|n| n.into_normalized()).collect()
    }
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

        let b0 = 1f64 - b2 - b1;

        let (uv0, uv1, uv2) = match self.mesh.uvs.as_ref() {
            Some(uvs) => (uvs[i0], uvs[i1], uvs[i2]),
            None => (Uv(0f64, 0f64), Uv(1f64, 0f64), Uv(1f64, 1f64))
        };

        let (u_axis, v_axis) = {
            let (du0, du1, dv0, dv1) = (uv0.0 - uv2.0, uv1.0 - uv2.0, uv0.1 - uv2.1, uv1.1 - uv2.1);
            let (dp0, dp1) = (p0 - p2, p1 - p2);
            let d = du0 * dv1 - dv0 * du1;
            // TODO: If d == 0, the user supplied degenerate UVs and we have to synthesize three basis vectors.
            (
                (dp0 *  dv1 - dp1 * dv0) / d,
                (dp0 * -du1 + dp1 * du0) / d,
            )
        };

        let shading_geometry = self.mesh.normals.as_ref().map(|normals| {
            let (n0, n1, n2) = (normals[i0], normals[i1], normals[i2]);
            // Blergh, can't overload .cross, so we have to convert the normal into a vector.
            let shading_normal = (n0 * b0 + n1 * b1 + n2 * b2).into_vector().into_normalized();
            let intermediate_u_axis = u_axis.into_normalized();
            let shading_v_axis = shading_normal.cross(intermediate_u_axis).into_normalized();
            IntersectionGeometry::new(
                shading_v_axis.cross(shading_normal),
                shading_v_axis,
            )
        });

        Some(Intersection {
            distance: t,
            location: ray.at(t),
            geometry: IntersectionGeometry::new(u_axis, v_axis),
            shading_geometry,
            uv: uv0 * b0 + uv1 * b1 + uv2 * b2,
            material: None,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    lazy_static! {
        static ref SINGLE_TRIANGLE: TriangleMesh = TriangleMeshData::new(
            vec![Point::new(-1f64, -1f64, 0f64), Point::new(1f64, -1f64, 0f64), Point::new(0f64, 1f64, 0f64)],
            Smoothing::None,
            None,
            vec![(0, 1, 2)],
        ).into_triangle_mesh();
    }

    #[test]
    fn it_should_intersect_a_half_infinite_ray() {
        let r = Ray::half_infinite(Point::new(0f64, 0f64, -3f64), Vec3::Z_AXIS);
        let i = SINGLE_TRIANGLE.intersect(&r);
        assert!(i.is_some());
        assert_eq!(i.unwrap().distance, 3f64);
    }

    #[test]
    fn it_should_intersect_a_finite_ray() {
        let r = Ray::finite(Point::new(0f64, 0f64, -3f64), Vec3::Z_AXIS, 0f64, 6f64);
        let i = SINGLE_TRIANGLE.intersect(&r);
        assert!(i.is_some());
        assert_eq!(i.unwrap().distance, 3f64);
    }

    #[test]
    fn it_should_not_intersect_a_half_infinite_ray() {
        let r = Ray::half_infinite(Point::new(5f64, 0f64, -5f64), Vec3::Z_AXIS);
        assert!(SINGLE_TRIANGLE.intersect(&r).is_none());
    }

    #[test]
    fn it_should_not_intersect_a_finite_ray() {
        let r = Ray::finite(Point::new(0f64, 0f64, -5f64), Vec3::Z_AXIS, 0f64, 1f64);
        assert!(SINGLE_TRIANGLE.intersect(&r).is_none());
    }
}
