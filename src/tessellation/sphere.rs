use math::*;
use geometry::*;
use image::RgbImage;
use std::collections::HashMap;

pub struct DisplacementMap {
    map: RgbImage,
    min: f64,
    max: f64,
}

const P0: Point = Point { x: 0f64, y: 0f64, z: 1f64 };
const P1: Point = Point { x: 1f64, y: 0f64, z: 0f64 };
const P2: Point = Point { x: 0f64, y: 1f64, z: 0f64 };
const P3: Point = Point { x: -1f64, y: 0f64, z: 0f64 };
const P4: Point = Point { x: 0f64, y: -1f64, z: 0f64 };
const P5: Point = Point { x: 0f64, y: 0f64, z: -1f64 };

pub fn tessellate_sphere(depth: u32, smoothing: Smoothing) -> TriangleMesh {
    let (positions, indices) = combine_and_offset(vec![
        divide_triangle((P2, P5, P1), 0, depth),
        divide_triangle((P2, P1, P0), 0, depth),
        divide_triangle((P2, P0, P3), 0, depth),
        divide_triangle((P2, P3, P5), 0, depth),
        divide_triangle((P4, P5, P3), 0, depth),
        divide_triangle((P4, P3, P0), 0, depth),
        divide_triangle((P4, P0, P1), 0, depth),
        divide_triangle((P4, P1, P5), 0, depth),
    ], get_hash_precision(depth));

    TriangleMesh::new(positions, smoothing, None, indices, true)
}

fn get_hash_precision(depth: u32) -> f64 {
    1f64 + 2f64.powi(depth as i32)
}

fn divide_triangle(triangle: (Point, Point, Point), current_depth: u32, depth_limit: u32)
                   -> (Vec<Point>, Vec<TriangleIndices>) {
    if current_depth >= depth_limit {
        (vec![triangle.0, triangle.1, triangle.2], vec![(0, 1, 2)])
    } else {
        let midpoints = (
            (triangle.0 + triangle.1).as_vector().as_normalized().as_point(),
            (triangle.1 + triangle.2).as_vector().as_normalized().as_point(),
            (triangle.2 + triangle.0).as_vector().as_normalized().as_point(),
        );
        let next_depth = current_depth + 1;
        combine_and_offset(vec![
            divide_triangle((triangle.0, midpoints.0, midpoints.2), next_depth, depth_limit),
            divide_triangle((triangle.1, midpoints.1, midpoints.0), next_depth, depth_limit),
            divide_triangle((triangle.2, midpoints.2, midpoints.1), next_depth, depth_limit),
            divide_triangle((midpoints.0, midpoints.1, midpoints.2), next_depth, depth_limit),
        ], get_hash_precision(depth_limit))
    }
}

// f64 not hashable ._.
// Move to xyz.rz?
#[derive(Debug, Eq, PartialEq, Hash)]
pub struct HashableXyz {
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
fn combine_and_offset(inputs: Vec<(Vec<Point>, Vec<TriangleIndices>)>, hash_precision: f64) -> (Vec<Point>, Vec<TriangleIndices>) {
    let mut positions = Vec::<Point>::new();
    let mut indices = Vec::<TriangleIndices>::new();
    let mut seen_points = HashMap::<HashableXyz, usize>::new();
    for (current_positions, current_indices) in inputs {
        let mut local_points_mapping = HashMap::<usize, usize>::new();
        for (local_index, position) in current_positions.iter().enumerate() {
            let hashed_position = as_hashable(position, hash_precision);
            let seen_point = seen_points.get(&hashed_position).map(|i| *i);
            let index = match seen_point {
                Some(i) => i,
                None => {
                    let i = positions.len();
                    positions.push(*position);
                    seen_points.insert(hashed_position, i);
                    i
                }
            };
            local_points_mapping.insert(local_index, index);
        }

        for triangle in &current_indices {
            indices.push((
                *local_points_mapping.get(&triangle.0).unwrap(),
                *local_points_mapping.get(&triangle.1).unwrap(),
                *local_points_mapping.get(&triangle.2).unwrap(),
            ));
        }
    }
    (positions, indices)
}
