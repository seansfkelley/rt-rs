#![allow(dead_code)]
use core::Ray;
use math::*;

pub static IDENTITY_TRANSFORM: Transform = Transform {
    object_to_world: IDENTITY_MATRIX,
    world_to_object: IDENTITY_MATRIX,
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transform {
    pub object_to_world: Mat4,
    pub world_to_object: Mat4,
}

impl Transform {
    pub fn new(object_to_world: Mat4) -> Transform {
        Transform {
            object_to_world,
            world_to_object: object_to_world.invert().unwrap(),
        }
    }
}

pub trait Transformable {
    fn object_to_world(&self, transform: &Transform) -> Self;
    fn world_to_object(&self, transform: &Transform) -> Self;
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

impl Transformable for Vec3 {
    fn object_to_world(&self, transform: &Transform) -> Vec3 {
        transform_vec3(self, &transform.object_to_world)
    }

    fn world_to_object(&self, transform: &Transform) -> Vec3 {
        transform_vec3(self, &transform.world_to_object)
    }
}

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

impl Transformable for Point {
    fn object_to_world(&self, transform: &Transform) -> Point {
        transform_point(self, &transform.object_to_world)
    }

    fn world_to_object(&self, transform: &Transform) -> Point {
        transform_point(self, &transform.world_to_object)
    }
}

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

impl Transformable for Normal {
    fn object_to_world(&self, transform: &Transform) -> Normal {
        transform_normal(self, &transform.object_to_world)
    }

    fn world_to_object(&self, transform: &Transform) -> Normal {
        transform_normal(self, &transform.world_to_object)
    }
}

impl Transformable for Ray {
    fn object_to_world(&self, transform: &Transform) -> Ray {
        Ray {
            origin: self.origin.object_to_world(transform),
            direction: self.direction.object_to_world(transform),
        }
    }

    fn world_to_object(&self, transform: &Transform) -> Ray {
        Ray {
            origin: self.origin.world_to_object(transform),
            direction: self.direction.world_to_object(transform),
        }
    }
}
