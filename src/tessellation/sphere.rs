use core::*;
use math::*;
use geometry::*;
use image::{RgbImage};
use std::collections::HashSet;
use std::collections::HashMap;
use std::iter::FromIterator;

pub struct DisplacementMap {
    map: RgbImage,
    min: f64,
    max: f64,
}

//pub fn displace_sphere(map: DisplacementMap,
//                                         sphere: TriangleMesh) -> TriangleMesh {}


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
    ]);

    TriangleMesh::new(positions, smoothing, None, indices, true)
}

fn divide_triangle(triangle: (Point, Point, Point), current_depth: u32, depth_limit: u32)
                   -> (Vec<Point>, Vec<TriangleIndices>) {
    if current_depth == depth_limit {
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
        ])
    }
}

fn combine_and_offset(mut inputs: Vec<(Vec<Point>, Vec<TriangleIndices>)>) -> (Vec<Point>, Vec<TriangleIndices>) {
    match inputs.pop() {
        Some(first) => {
            let mut positions = first.0.clone();
            let mut indices = first.1.clone();
            let mut seen_points = HashSet::from_iter(positions);
            let mut offset = positions.len();
            for (current_positions, current_indices) in inputs {
                
            }
            (positions, indices)
        }
        None => (vec![], vec![])
    }
}
