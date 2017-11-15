#![allow(dead_code)]
use core::{ Ray, Camera };
use math::*;

pub static IDENTITY_TRANSFORM: Transform = Transform {
    m: IDENTITY_MATRIX,
    m_inverse: IDENTITY_MATRIX,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub m: Mat4,
    pub m_inverse: Mat4,
}

impl Transform {
    pub fn new(m: Mat4) -> Transform {
        Transform {
            m,
            m_inverse: m.invert().unwrap(),
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

// pbrt pg. 86
fn transform_vec3(in_vector: &Vec3, mat4: &Mat4) -> Vec3 {
    // The homogenous coordinate is implicitly zero, i.e., vectors are not translatable.
    let mut vec3 = [0f64; 3];

    for i in 0..3 {
        vec3[i] =
            mat4.get_cell(0, i) * in_vector.x +
            mat4.get_cell(1, i) * in_vector.y +
            mat4.get_cell(2, i) * in_vector.z;
    }

    Vec3::new(vec3[0], vec3[1], vec3[2])
}

make_transformable!(Vec3, transform_vec3);

// pbrt pg. 86
fn transform_point(point: &Point, mat4: &Mat4) -> Point {
    let mut vec4 = [0f64; 4];

    for i in 0..4 {
        vec4[i] =
            mat4.get_cell(0, i) * point.x +
            mat4.get_cell(1, i) * point.y +
            mat4.get_cell(2, i) * point.z +
            mat4.get_cell(3, i);
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
fn transform_normal(normal: &Normal, mat4: &Mat4) -> Normal {
    let mut vec3 = [0f64; 3];

    for i in 0..3 {
        // Note, per pbrt, that we don't compute the transpose but just swap i/j indices.
        vec3[i] =
            mat4.get_cell(i, 0) * normal.x +
            mat4.get_cell(i, 1) * normal.y +
            mat4.get_cell(i, 2) * normal.z;
    }

    Normal {
        x: vec3[0],
        y: vec3[1],
        z: vec3[2],
    }
}

make_transformable!(Normal, transform_normal);

impl Transformable for Ray {
    fn transform(&self, transform: &Transform) -> Ray {
        Ray {
            origin: self.origin.transform(transform),
            direction: self.direction.transform(transform),
        }
    }

    fn invert_transform(&self, transform: &Transform) -> Ray {
        Ray {
            origin: self.origin.invert_transform(transform),
            direction: self.direction.invert_transform(transform),
        }
    }
}

impl Transformable for Camera {
    fn transform(&self, transform: &Transform) -> Camera {
        Camera {
            position: self.position.transform(transform),
            up: self.up.transform(transform),
            direction: self.direction.transform(transform),
            right: self.right.transform(transform),
        }
    }

    fn invert_transform(&self, transform: &Transform) -> Camera {
        Camera {
            position: self.position.invert_transform(transform),
            up: self.up.invert_transform(transform),
            direction: self.direction.invert_transform(transform),
            right: self.right.invert_transform(transform),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    mod vec3 {
        use super::*;

        #[test]
        fn it_should_not_translate_the_vector() {
            let vec3 = Vec3::new(1f64, 2f64, 3f64);
            assert_eq!(vec3.transform(Transform::new(Mat4::create_scale())));
        }

// describe! transform {
//     describe! vec3_transformation {
//         it "should not change the vector" {
//             let transform = Transform::new(IDENTITY_MATRIX);
//             let expected = Vec3::new(1f64, 2f64, 3f64);
//             let actual = expected.transform(&transform);
//             assert_eq!(actual, expected);
//         }

//         it "should scale the vector" {
//             let v = Vec3::new(2f64, 3f64, 4f64);
//             let matrix = Mat4::create_scale(v);
//             let transform = Transform::new(matrix);
//             let expected = Vec3::new(2f64, 6f64, 12f64);
//             let actual = expected.transform(&transform);
//             assert_eq!(actual, expected);
//         }

//         // it "should rotate the vector" {

//         // }

//         // it "should not translate the vector" {

//         // }
//     }
// }
    }
}
