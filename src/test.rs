#![cfg(test)]

use transform::{Mat4, IDENTITY};
use vector::Vec3;
use std::f64::consts::PI;

static TEST_MATRIX: Mat4 = Mat4 {
    cells: [
        [1f64, 2f64, 3f64, 4f64],
        [5f64, 6f64, 7f64, 8f64],
        [9f64, 10f64, 11f64, 12f64],
        [13f64, 14f64, 15f64, 16f64],
    ],
};

static INVERTIBLE_TEST_MATRIX: Mat4 = Mat4 {
    cells: [
        [2f64, 4f64, 8f64, 16f64],
        [3f64, 9f64, 27f64, 81f64],
        [4f64, 16f64, 64f64, 256f64],
        [5f64, 25f64, 125f64, 625f64],
    ],
};

static X_AXIS: Vec3 = Vec3 { x: 1f64, y: 0f64, z: 0f64 };
static Y_AXIS: Vec3 = Vec3 { x: 0f64, y: 1f64, z: 0f64 };
static TEST_VECTOR: Vec3 = Vec3 { x: 1f64, y: 2f64, z: 3f64 };

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
            assert_eq!(Mat4::create_rotation(PI, &X_AXIS) * TEST_VECTOR, expected);
        }

        it "should rotate around y-axis" {
            let transform = Mat4::create_rotation(PI, &Y_AXIS);
            let start = Vec3 { x: 1f64, y: -2f64, z: -3f64 };
            let expected = Vec3 { x: -1.0000000000000004f64, y: -2f64, z: 3f64 };
            assert_eq!(transform * start, expected);
        }

        it "should compose rotations" {
            let transform = Mat4::create_rotation(PI, &X_AXIS).rotate(PI, &Y_AXIS);
            let expected = Vec3 { x: -0.9999999999999997f64, y: -1.9999999999999996f64, z: 3.0000000000000004f64 };
            assert_eq!(transform * TEST_VECTOR, expected);
        }
    }
}
