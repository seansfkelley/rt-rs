use std::collections::HashMap;
use math::*;
use core::*;
use geometry::*;

pub fn tessellate_sphere(depth: u32, smoothing: Smoothing) -> TriangleMeshData {
    const P0: Point = Point { x:  0f64, y:  0f64, z:  1f64 };
    const P1: Point = Point { x:  1f64, y:  0f64, z:  0f64 };
    const P2: Point = Point { x:  0f64, y:  1f64, z:  0f64 };
    const P3: Point = Point { x: -1f64, y:  0f64, z:  0f64 };
    const P4: Point = Point { x:  0f64, y: -1f64, z:  0f64 };
    const P5: Point = Point { x:  0f64, y:  0f64, z: -1f64 };

    let expected_positions = 6usize * 3usize.pow(depth);
    let expected_triangles = 8usize * 4usize.pow(depth);
    let hash_precision = 1f64 + 2f64.powi(depth as i32);
    let mut builder = TriangleMeshBuilder::new_with_expected_size(hash_precision, expected_positions, expected_triangles);
    // TODO: move to lazy static
    let starting_triangles = vec![
        (P2, P5, P1),
        (P2, P1, P0),
        (P2, P0, P3),
        (P2, P3, P5),
        (P4, P5, P3),
        (P4, P3, P0),
        (P4, P0, P1),
        (P4, P1, P5),
    ];
    for triangle in starting_triangles {
        divide_triangle(triangle, &mut builder, 0, depth);
    }

    builder.build(smoothing)
}

fn divide_triangle(triangle: (Point, Point, Point), builder: &mut TriangleMeshBuilder, current_depth: u32, depth_limit: u32) {
    if current_depth >= depth_limit {
        builder.add_triangles(
            &vec![triangle.0, triangle.1, triangle.2],
            &vec![
                sphere_uv_for_normalized_point(triangle.0),
                sphere_uv_for_normalized_point(triangle.1),
                sphere_uv_for_normalized_point(triangle.2),
            ],
            &vec![(0, 1, 2)],
        );
    } else {
        let midpoints = (
            (triangle.0 + triangle.1).into_vector().into_normalized().into_point(),
            (triangle.1 + triangle.2).into_vector().into_normalized().into_point(),
            (triangle.2 + triangle.0).into_vector().into_normalized().into_point(),
        );
        let next_depth = current_depth + 1;
        let next_triangles = vec![
            (triangle.0, midpoints.0, midpoints.2),
            (triangle.1, midpoints.1, midpoints.0),
            (triangle.2, midpoints.2, midpoints.1),
            (midpoints.0, midpoints.1, midpoints.2),
        ];
        for next_triangle in next_triangles {
            divide_triangle(next_triangle, builder, next_depth, depth_limit);
        }
    }
}

// f64 not hashable ._.
// Move to xyz.rz?
#[derive(Debug, Eq, PartialEq, Hash)]
struct HashableXyz {
    pub x: usize,
    pub y: usize,
    pub z: usize,
}

fn as_hashable<T: Xyz>(xyz: &T, precision: f64) -> HashableXyz {
    HashableXyz {
        x: (xyz.x() * precision) as usize,
        y: (xyz.y() * precision) as usize,
        z: (xyz.z() * precision) as usize,
    }
}

// TODO: Should probably be moved into triangle_mesh.rs
struct TriangleMeshBuilder {
    precision: f64,
    positions: Vec<Point>,
    uvs: Vec<Uv>,
    indices: Vec<TriangleIndices>,
    point_mapping: HashMap<HashableXyz, usize>,
}

impl TriangleMeshBuilder {
    pub fn new_with_expected_size(precision: f64, expected_positions: usize, expected_triangles: usize) -> TriangleMeshBuilder {
        TriangleMeshBuilder {
            precision,
            positions: Vec::with_capacity(expected_positions),
            uvs: Vec::with_capacity(expected_positions),
            indices: Vec::with_capacity(expected_triangles),
            point_mapping: HashMap::with_capacity(expected_positions),
        }
    }

    pub fn add_triangles(&mut self, positions: &[Point], uvs: &[Uv], indices: &[TriangleIndices]) {
        let mut local_points_mapping = HashMap::<usize, usize>::new();

        for (local_index, position) in positions.iter().enumerate() {
            let hashed_position = as_hashable(position, self.precision);
            let seen_point = self.point_mapping.get(&hashed_position).map(|p| p.clone());
            let index = match seen_point {
                Some(p) => p,
                None => {
                    let p = self.positions.len();
                    self.positions.push(*position);
                    self.uvs.push(uvs[local_index]);
                    self.point_mapping.insert(hashed_position, p);
                    p
                }
            };
            local_points_mapping.insert(local_index, index);
        }

        for triangle in indices {
            self.indices.push((
                *local_points_mapping.get(&triangle.0).unwrap(),
                *local_points_mapping.get(&triangle.1).unwrap(),
                *local_points_mapping.get(&triangle.2).unwrap(),
            ));
        }
    }

    pub fn build(self, smoothing: Smoothing) -> TriangleMeshData {
        TriangleMeshData::new(self.positions, smoothing, Some(self.uvs), self.indices)
    }
}

