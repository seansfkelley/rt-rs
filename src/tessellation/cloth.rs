use math::*;
use geometry::*;

// TODO: find better names for these
#[derive(PartialEq)]
pub enum ClothClosure {
    None,
    Cap,
    Join,
    Closed,
}

impl ClothClosure {
    pub fn should_cap(&self) -> bool {
        *self == ClothClosure::Cap || *self == ClothClosure::Closed
    }

    pub fn should_join(&self) -> bool {
        *self == ClothClosure::Join || *self == ClothClosure::Closed
    }
}

// TODO: Switch to TriangleMeshBuilder?
// Would be slower but much simpler
pub fn create_cloth(curves: Vec<Box<Curve>>, tessellation_factor: usize, closure: ClothClosure) -> TriangleMeshData {
    let number_of_curves = curves.len();
    if closure.should_cap() || closure.should_join() {
        assert!(number_of_curves > 2);
    }

    let total_positions = tessellation_factor * number_of_curves;
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
    let mut total_triangles = triangles_per_curve_pair * (number_of_curves - 1);
    if closure.should_cap() {
        total_triangles += 2 * (number_of_curves - 2);
    }
    if closure.should_join() {
        total_triangles += triangles_per_curve_pair;
    }

    let connect_curves = |indices: &mut Vec<TriangleIndices>, left: usize, right: usize| {
        let left_curve_start = left * tessellation_factor;
        let right_curve_start = right * tessellation_factor;
        for i in 0..squares_per_curve_pair {
            let a = left_curve_start + i + 1;
            let b = left_curve_start + i;
            let c = right_curve_start + i;
            let d = right_curve_start + i + 1;
            indices.push((b, a, c));
            indices.push((c, a, d));
        }
    };

    let ref mut indices = Vec::<TriangleIndices>::with_capacity(total_triangles);
    for curve_index in 0..(number_of_curves - 1) {
        connect_curves(indices, curve_index, curve_index + 1);
    }

    if closure.should_join() {
        connect_curves(indices, number_of_curves - 1, 0);
    }

    if closure.should_cap() {
        for curve_index in 1..(number_of_curves - 1) {
            indices.push((
                0,
                curve_index * tessellation_factor,
                (curve_index + 1) * tessellation_factor
            ));
            indices.push((
                tessellation_factor - 1,
                (curve_index + 1) * tessellation_factor - 1,
                (curve_index + 2) * tessellation_factor - 1
            ));
        }
    }

    TriangleMeshData::new(positions, Smoothing::Implicit, None, indices.to_owned(), closure == ClothClosure::Closed)
}
