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
    // FYI, the "front" is when the vertices are in counterclockwise order.
    // > By default, counterclockwise polygons are taken to be front-facing.
    // (https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glFrontFace.xml)
    pub fn new(positions: Vec<Point>, normals: Vec<Normal>, uvs: Vec<Uv>, indices: Vec<TriangleIndices>) -> TriangleMesh {
        // TODO: When we actually do UVs and given normals, do this.
        // assert_eq!(positions.len(), normals.len());
        // assert_eq!(positions.len(), uvs.len());
        // TODO: Also check that the coordinates are in-bounds.
        TriangleMesh { positions, normals, uvs, indices }
    }

    fn intersect_triplet(&self, triplet: &TriangleIndices, ray: &Ray) -> Option<Hit> {
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

        return Some(Hit {
            enter: Some(Intersection {
                distance: t,
                location: ray.at(t),
                normal: e1.cross(e2).as_normalized().as_normal(),
                uv: (0f64, 0f64),
            }),
            exit: Intersection {
                distance: t,
                location: ray.at(t),
                normal: e1.cross(e2).as_normalized().as_normal(),
                uv: (0f64, 0f64),
            },
            debug: false,
        });
    }
}

impl Geometry for TriangleMesh {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        for triplet in &self.indices {
            match self.intersect_triplet(&triplet, ray) {
                Some(hit) => { return Some(hit); }
                None => {}
            }
        }

        return None;
    }
}
