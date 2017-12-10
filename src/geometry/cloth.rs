use math::*;
use geometry::*;

pub fn create_cloth(curves: Vec<Box<Curve>>, tessellation_factor: usize) -> TriangleMesh {
    let total_positions = tessellation_factor * curves.len();
    let mut positions = Vec::<Point>::with_capacity(total_positions);
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
    let mut normals = vec![Normal::uniform(0f64); total_positions];
    for curve_index in 0..(curves.len() - 1) {
        let left_curve_start = curve_index * tessellation_factor;
        let right_curve_start = left_curve_start + tessellation_factor;
        for i in 0..squares_per_curve_pair {
            let a = left_curve_start + i + 1;
            let b = left_curve_start + i;
            let c = right_curve_start + i;
            let d = right_curve_start + i + 1;
            indices.push((a, b, c));
            indices.push((a, c, d));
            let first_triangle_normal = (positions[c] - positions[a]).cross(positions[b] - positions[a]).as_normalized().as_normal();
            let second_triangle_normal = (positions[d] - positions[a]).cross(positions[c] - positions[a]).as_normalized().as_normal();
            normals[a] = normals[a] + first_triangle_normal + second_triangle_normal;
            normals[b] = normals[b] + first_triangle_normal;
            normals[c] = normals[c] + first_triangle_normal + second_triangle_normal;
            normals[d] = normals[d] + second_triangle_normal;
        }
    }

    TriangleMesh::new(positions,
                      Some(normals.iter().map(|n| n.as_normalized()).collect()),
                      None, indices, false)
}
