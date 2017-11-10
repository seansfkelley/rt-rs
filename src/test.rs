#![cfg(test)]

use transform::{Mat4, IDENTITY, X_AXIS, Y_AXIS};
use core::Vec3;
use std;
use std::f64::consts::PI;
use geometry::*;
use std::rc::Rc;
use material;
use color::Color;

pub const ORIGIN: Vec3 = Vec3 { x: 0f64, y: 0f64, z: 0f64 };

const TEST_MATRIX: Mat4 = Mat4 {
    cells: [
        [1f64, 2f64, 3f64, 4f64],
        [5f64, 6f64, 7f64, 8f64],
        [9f64, 10f64, 11f64, 12f64],
        [13f64, 14f64, 15f64, 16f64],
    ],
};

const INVERTIBLE_TEST_MATRIX: Mat4 = Mat4 {
    cells: [
        [2f64, 4f64, 8f64, 16f64],
        [3f64, 9f64, 27f64, 81f64],
        [4f64, 16f64, 64f64, 256f64],
        [5f64, 25f64, 125f64, 625f64],
    ],
};

const TEST_VECTOR: Vec3 = Vec3 { x: 1f64, y: 2f64, z: 3f64 };

const YELLOW_MATTE: material::FlatMaterial = material::FlatMaterial { color: Color { r: 0.7f64, g: 0.7f64, b: 0f64 }, specular_exponent: 0f64, reflectivity: 0f64 };
const STRAIGHT_RAY: Ray = Ray { origin: Vec3 { x: 0f64, y: 0f64, z: 5f64 }, direction: Vec3 { x: 0f64, y: 0f64, z: -1f64 } };
const OFFSET_RAY: Ray = Ray { origin: Vec3 { x: 5f64, y: 0f64, z: 5f64 }, direction: Vec3 { x: 0f64, y: 0f64, z: -1f64 } };

describe! mat4 {
    it "should start with zeroes" {
        let empty = Mat4::create();
        assert_eq!(empty.get_cell(0, 0), 0f64);
    }

    describe! matrix_multiplication {
        it "should multiply" {
            let expected = Mat4 { cells: [[90f64, 100f64, 110f64, 120f64], [202f64, 228f64, 254f64, 280f64], [314f64, 356f64, 398f64, 440f64], [426f64, 484f64, 542f64, 600f64]] };
            assert_eq!(TEST_MATRIX * TEST_MATRIX, expected);
        }

        it "should identity multiply" {
            assert_eq!(TEST_MATRIX * IDENTITY, TEST_MATRIX);
        }
    }

    describe! vector_multiplication {
        it "should multiply" {
            let expected = Vec3::new(18f64, 46f64, 74f64);
            assert_eq!(TEST_MATRIX * TEST_VECTOR, expected);
        }

        it "should identity multiply" {
            assert_eq!(IDENTITY * TEST_VECTOR, TEST_VECTOR);
        }
    }

    describe! inverse {
        it "should be empty for undefined inversions" {
            assert_eq!(TEST_MATRIX.invert(), None);
        }

        it "should be identity for identity" {
            assert_eq!(IDENTITY.invert(), Some(IDENTITY));
        }

        it "should invert" {
            let expected: Mat4 = Mat4 {
                cells: [
                    [5f64, (-20f64 / 3f64), (15f64 / 4f64), (-4f64 / 5f64)],
                    [(-47f64 / 12f64), (19f64/3f64), (-31f64 / 8f64), (13f64/15f64)],
                    [1f64, (-11f64/6f64), (5f64/4f64), (-3f64/10f64)],
                    [(-1f64/12f64), (1f64/6f64), (-1f64/8f64), (1f64/30f64)],
                ],
            };
            assert_eq!(INVERTIBLE_TEST_MATRIX.invert(), Some(expected));
        }
    }

    describe! rotation {
        it "should rotate around x-axis" {
            // #floats
            let expected = Vec3 { x: 1f64, y: -2.0000000000000004f64, z: -2.9999999999999996f64 };
            assert_eq!(Mat4::create_rotation(PI, X_AXIS) * TEST_VECTOR, expected);
        }

        it "should rotate around x-axis at different angles" {
            let expected = Vec3 { x: 4f64, y: -6.392304845413263f64, z: 12.92820323027551f64 };
            assert_eq!(Mat4::create_rotation(PI / 3f64, X_AXIS) * (TEST_VECTOR * 4f64), expected);
        }

        it "should rotate around y-axis" {
            let transform = Mat4::create_rotation(PI, Y_AXIS);
            let start = Vec3 { x: 1f64, y: -2f64, z: -3f64 };
            let expected = Vec3 { x: -1.0000000000000004f64, y: -2f64, z: 3f64 };
            assert_eq!(transform * start, expected);
        }

        it "should compose rotations" {
            let transform = Mat4::create_rotation(PI, X_AXIS).rotate(PI, Y_AXIS);
            let expected = Vec3 { x: -0.9999999999999997f64, y: -1.9999999999999996f64, z: 3.0000000000000004f64 };
            assert_eq!(transform * TEST_VECTOR, expected);
        }

        it "should be reversed by inverse" {
            let transform = Mat4::create_rotation(PI, X_AXIS).rotate(PI, Y_AXIS);
            let inverse = transform.invert().unwrap();
            assert_eq!(inverse * (transform * TEST_VECTOR), TEST_VECTOR);
        }
    }

    describe! translation {
        it "should translate" {
            let expected = Vec3 { x: 2f64, y: 2f64, z: 3f64 };
            assert_eq!(Mat4::create_translation(X_AXIS) * TEST_VECTOR, expected);
        }

        it "should compose translations" {
            let expected = Vec3 { x: 3f64, y: 4f64, z: 6f64 };
            let transform = Mat4::create_translation(X_AXIS)
                .translate(TEST_VECTOR);
            assert_eq!(transform * TEST_VECTOR, expected);
        }
    }

    describe! scale {
        it "should scale" {
            let expected = Vec3 { x: 1f64, y: 0f64, z: 0f64 };
            assert_eq!(Mat4::create_scale(X_AXIS) * TEST_VECTOR, expected);
        }

        it "should compose scales" {
            let expected = Vec3 { x: 2f64, y: 0.5f64, z: 0f64 };
            let transform = Mat4::create_scale(Vec3 { x: 2f64, y: 0.5f64, z: 1f64 })
                .scale(Vec3 { x: 1f64, y: 0.5f64, z: 0f64 });
            assert_eq!(transform * TEST_VECTOR, expected);
        }
    }

    describe! composition {
        it "should compose" {
            let expected = Vec3 { x: 10f64, y: -6.000000000000002f64, z: -6.999999999999999f64 };
            let transform = Mat4::create_scale(Vec3 { x: 2f64, y: 1f64, z: 1f64 })
                .rotate(PI, X_AXIS)
                .translate(Vec3 { x: 4f64, y: 4f64, z: 4f64 });
            assert_eq!(transform * TEST_VECTOR, expected);
        }
    }
}

describe! ray {
    it "should transform" {
        let transform = Mat4::create_rotation(PI, X_AXIS);
        let ray = Ray::new(ORIGIN, Vec3::new(0f64, 0f64, -1f64));
        let transformed_ray = ray.transform(transform, transform.without_scale());
        assert_eq!(transformed_ray.origin, ORIGIN);
        assert_eq!(transformed_ray.direction, Vec3::new(0f64, 0.00000000000000012246467991473532f64, 1f64));
    }

    it "should transform and direction should ignore translation" {
        let transform = Mat4::create_rotation(PI, X_AXIS)
            .translate(X_AXIS);
        let ray = Ray::new(ORIGIN, Vec3::new(0f64, 0f64, -1f64));
        let transformed_ray = ray.transform(transform, transform.without_scale());
        assert_eq!(transformed_ray.origin, X_AXIS);
        assert_eq!(transformed_ray.direction, Vec3::new(0f64, 0.00000000000000012246467991473532f64, 1f64));
    }
}

describe! sphere {
    it "should simply intersect" {
        let yellow_matte: Rc<material::Material> = Rc::new(YELLOW_MATTE);
        let sphere = Sphere::new(1f64, IDENTITY, &yellow_matte);
        assert!(sphere.intersect(&STRAIGHT_RAY).is_some());
        assert!(sphere.intersect(&OFFSET_RAY).is_none());
    }

    it "should intersect translations" {
        let yellow_matte: Rc<material::Material> = Rc::new(YELLOW_MATTE);
        let sphere = Sphere::new(1f64, Mat4::create_translation(Vec3::new(5f64, 0f64, 0f64)), &yellow_matte);
        assert!(sphere.intersect(&STRAIGHT_RAY).is_none());
        assert!(sphere.intersect(&OFFSET_RAY).is_some());
    }
}
