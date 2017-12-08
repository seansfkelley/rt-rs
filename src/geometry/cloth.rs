use math::*;
use geometry::*;

pub fn create_cloth(curves: Vec<Box<Curve>>, tessellation_factor: usize) -> TriangleMesh {
    let mut positions = Vec::<Point>::with_capacity(tessellation_factor * curves.len());
    let curve_fraction = tessellation_factor as f64 - 1f64;
    for curve in &curves {
        for i in 0..tessellation_factor {
            let t = i as f64 / curve_fraction;
            positions.push(curve.at(t));
        }
    }

    let triangles_per_curve_pair = 2 * tessellation_factor - 2;
    let squares_per_curve_pair = tessellation_factor - 1;
    let mut indices = Vec::<TriangleIndices>::with_capacity(triangles_per_curve_pair * (curves.len() - 1));
    for curve_index in 0..(curves.len() - 1) {
        let left_curve_start = curve_index * tessellation_factor;
        let right_curve_start = left_curve_start + tessellation_factor;
        for i in 0..squares_per_curve_pair {
            indices.push((left_curve_start + i + 1, left_curve_start + i, right_curve_start + i));
            indices.push((left_curve_start + i + 1, right_curve_start + i, right_curve_start + i + 1));
        }
    }

    TriangleMesh::new(positions, vec![], vec![], indices)
}
