use math::*;

pub static IDENTITY_TRANSFORM: Transform = Transform {
    m: IDENTITY_MATRIX,
    m_inverse: IDENTITY_MATRIX,
    swaps_handedness: false,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub m: Mat4,
    pub m_inverse: Mat4,
    pub swaps_handedness: bool,
}

fn determinant_3x3(m: &Mat4) -> f64 {
    (m.cells[0][0] *
        (m.cells[1][1] * m.cells[2][2] - m.cells[1][2] * m.cells[2][1])) -
    (m.cells[0][1] *
        (m.cells[1][0] * m.cells[2][2] - m.cells[1][2] * m.cells[2][0])) +
    (m.cells[0][2] *
        (m.cells[1][0] * m.cells[2][1] - m.cells[1][1] * m.cells[2][0]))
}

impl Transform {
    pub fn new(m: Mat4) -> Transform {
        Transform {
            m,
            m_inverse: m.invert().unwrap(),
            swaps_handedness: determinant_3x3(&m) < 0f64,
        }
    }
}

pub trait Transformable {
    fn transform(&self, transform: &Transform) -> Self;
    fn invert_transform(&self, transform: &Transform) -> Self;
}

macro_rules! make_transformable {
    ($struct:ty, $transformer:ident) => {
        impl Transformable for $struct {
            fn transform(&self, transform: &Transform) -> $struct {
                $transformer(self, &transform.m)
            }

            fn invert_transform(&self, transform: &Transform) -> $struct {
                $transformer(self, &transform.m_inverse)
            }
        }
    };
}

macro_rules! make_transformable_inverted {
    ($struct:ty, $transformer:ident) => {
        impl Transformable for $struct {
            fn transform(&self, transform: &Transform) -> $struct {
                $transformer(self, &transform.m_inverse)
            }

            fn invert_transform(&self, transform: &Transform) -> $struct {
                $transformer(self, &transform.m)
            }
        }
    };
}

// pbrt pg. 86
fn transform_point(point: &Point, mat4: &Mat4) -> Point {
    let mut vec4 = [0f64; 4];

    for i in 0..4 {
        vec4[i] =
            mat4.cells[i][0] * point.x +
            mat4.cells[i][1] * point.y +
            mat4.cells[i][2] * point.z +
            mat4.cells[i][3];
    }

    // TODO: Worth optimizing away the division when it's == 1, per pbrt?
    Point {
        x: vec4[0] / vec4[3],
        y: vec4[1] / vec4[3],
        z: vec4[2] / vec4[3],
    }
}

make_transformable!(Point, transform_point);

// pbrt pg. 86
fn transform_vec3(in_vector: &Vec3, mat4: &Mat4) -> Vec3 {
    // The homogenous coordinate is implicitly zero, i.e., vectors are not translatable.
    let mut vec3 = [0f64; 3];

    for i in 0..3 {
        vec3[i] =
            mat4.cells[i][0] * in_vector.x +
            mat4.cells[i][1] * in_vector.y +
            mat4.cells[i][2] * in_vector.z;
    }

    Vec3::new(vec3[0], vec3[1], vec3[2])
}

make_transformable!(Vec3, transform_vec3);

// pbrt pg. 86
fn transform_normal(normal: &Normal, mat4: &Mat4) -> Normal {
    let mut vec3 = [0f64; 3];

    for i in 0..3 {
        // Note, per pbrt, that we don't compute the transpose but just swap i/j indices.
        vec3[i] =
            mat4.cells[0][i] * normal.x +
            mat4.cells[1][i] * normal.y +
            mat4.cells[2][i] * normal.z;
    }

    Normal {
        x: vec3[0],
        y: vec3[1],
        z: vec3[2],
    }
}

make_transformable_inverted!(Normal, transform_normal);

#[cfg(test)]
mod tests {
    use super::*;

    mod vec3 {
        use super::*;

        #[test]
        fn it_should_not_translate_the_vector() {
            let vec3 = Vec3::new(1f64, 2f64, 3f64);
            assert_eq!(vec3.transform(&Transform::new(Mat4::create_translation(Vec3::new(10f64, 20f64, 30f64)))), vec3);
        }

        #[test]
        fn it_should_scale_the_vector() {
            assert_eq!(
                Vec3::new(1f64, 2f64, 3f64).transform(&Transform::new(Mat4::create_scale(Vec3::new(10f64, 20f64, 30f64)))),
                Vec3::new(10f64, 40f64, 90f64)
            );
        }

        // #[test]
        // fn it_should_rotate_the_vector_around_x() {
        //     assert_eq!(
        //         Vec3::new(1f64, 2f64, 3f64).transform(&Transform::new(Mat4::create_scale(&(10f64, 20f64, 30f64)))),
        //         Vec3::new(10f64, 40f64, 90f64)
        //     );
        // }
    }
}
