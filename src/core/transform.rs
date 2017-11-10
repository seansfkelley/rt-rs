#![allow(dead_code)]
use core::Ray;
use math::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub matrix: Mat4,
    pub inverse: Mat4,
}

impl Transform {
    pub fn new(matrix: Mat4) -> Transform {
        Transform {
            matrix,
            inverse: matrix.invert().unwrap(),
        }
    }
}

pub trait Transformable {
    fn transform(&self, transform: &Transform) -> Self;
}

impl Transformable for Vec3 {
    // pbrt pg. 86
    fn transform(&self, transform: &Transform) -> Vec3 {
        // The homogenous coordinate is implicitly zero, i.e., vectors are not translatable.
        let mut vec3 = [0f64; 3];

        for i in 0..3 {
            vec3[i] =
                transform.matrix.get_cell(0, i) * self.x +
                transform.matrix.get_cell(1, i) * self.y +
                transform.matrix.get_cell(2, i) * self.z;
        }

        Vec3 {
            x: vec3[0],
            y: vec3[1],
            z: vec3[2],
        }
    }
}

impl Transformable for Point {
    // pbrt pg. 86
    fn transform(&self, transform: &Transform) -> Point {
        let mut vec4 = [0f64; 4];

        for i in 0..4 {
            vec4[i] =
                transform.matrix.get_cell(0, i) * self.x +
                transform.matrix.get_cell(1, i) * self.y +
                transform.matrix.get_cell(2, i) * self.z +
                transform.matrix.get_cell(3, i);
        }

        // TODO: Worth optimizing away the division when it's == 1, per pbrt?
        Point {
            x: vec4[0] / vec4[3],
            y: vec4[1] / vec4[3],
            z: vec4[2] / vec4[3],
        }
    }
}

impl Transformable for Normal {
    // pbrt pg. 86
    fn transform(&self, transform: &Transform) -> Normal {
        let mut vec3 = [0f64; 3];

        for i in 0..3 {
            // Note, per pbrt, that we don't compute the transpose but just swap i/j indices.
            vec3[i] =
                transform.inverse.get_cell(i, 0) * self.x +
                transform.inverse.get_cell(i, 1) * self.y +
                transform.inverse.get_cell(i, 2) * self.z;
        }

        Normal {
            x: vec3[0],
            y: vec3[1],
            z: vec3[2],
        }
    }
}

impl Transformable for Ray {
    // pbrt pg. 86
    fn transform(&self, transform: &Transform) -> Ray {
        Ray {
            origin: self.origin.transform(transform),
            direction: self.direction.transform(transform),
        }
    }
}
