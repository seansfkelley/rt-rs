use math::*;

pub static IDENTITY_TRANSFORM: Transform = Transform {
    m: IDENTITY_MATRIX,
    m_inverse: IDENTITY_MATRIX,
};

#[derive(Debug, Clone, PartialEq)]
pub struct Transform {
    pub m: Mat4,
    pub m_inverse: Mat4,
}

impl Transform {
    pub fn new(m: Mat4) -> Transform {
        let m_inverse = m.invert().expect("transforms should always be invertible");
        Transform { m, m_inverse }
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
    Point::new(
        vec4[0] / vec4[3],
        vec4[1] / vec4[3],
        vec4[2] / vec4[3],
    )
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

    Normal::new(
        vec3[0],
        vec3[1],
        vec3[2],
    )
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
