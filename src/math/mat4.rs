#![allow(dead_code)]
use std::ops::Mul;
use super::xyz::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
    // (row, column)
    pub cells: [[f64; 4]; 4],
}

pub const IDENTITY: Mat4 = Mat4 {
    cells: [
        [1f64, 0f64, 0f64, 0f64],
        [0f64, 1f64, 0f64, 0f64],
        [0f64, 0f64, 1f64, 0f64],
        [0f64, 0f64, 0f64, 1f64],
    ],
};

pub const X_AXIS: Vec3 = Vec3 { x: 1f64, y: 0f64, z: 0f64 };
pub const Y_AXIS: Vec3 = Vec3 { x: 0f64, y: 1f64, z: 0f64 };
pub const Z_AXIS: Vec3 = Vec3 { x: 0f64, y: 0f64, z: 1f64 };

impl Mat4 {
    pub fn create() -> Mat4 {
        Mat4 { cells: [[0f64; 4]; 4] }
    }

    pub fn create_translation(translation: Vec3) -> Mat4 {
        let mut cells = IDENTITY.cells.clone();
        cells[0][3] = translation.x;
        cells[1][3] = translation.y;
        cells[2][3] = translation.z;
        Mat4 { cells }
    }

    pub fn create_scale(scale: Vec3) -> Mat4 {
        let mut cells = [[0f64; 4]; 4];
        cells[0][0] = scale.x;
        cells[1][1] = scale.y;
        cells[2][2] = scale.z;
        cells[3][3] = 1f64;
        Mat4 { cells }
    }

    pub fn create_rotation(theta: f64, axis: Vec3) -> Mat4 {
        axis.assert_normalized();
        let mut cells = [[0f64; 4]; 4];
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        let one_minus_cos_theta = 1f64 - cos_theta;
        let one_minus_sin_theta = 1f64 - sin_theta;

        cells[0][0] = cos_theta + axis.x * axis.x * one_minus_cos_theta;
        cells[0][1] = axis.x * axis.y * one_minus_cos_theta - axis.z * sin_theta;
        cells[0][2] = axis.x * axis.z * one_minus_cos_theta + axis.y * sin_theta;

        cells[1][0] = axis.y * axis.x * one_minus_cos_theta + axis.z * sin_theta;
        cells[1][1] = cos_theta + axis.y * axis.y * one_minus_cos_theta;
        cells[1][2] = axis.y * axis.z * one_minus_cos_theta - axis.x * sin_theta;

        cells[2][0] = axis.z * axis.x * one_minus_cos_theta - axis.y * sin_theta;
        cells[2][1] = axis.z * axis.y * one_minus_cos_theta + axis.x * sin_theta;
        cells[2][2] = cos_theta + axis.z * axis.z * one_minus_cos_theta;

        cells[3][3] = 1f64;
        Mat4 { cells }
    }

    pub fn translate(&self, translation: Vec3) -> Mat4 {
        *self * Mat4::create_translation(translation)
    }

    pub fn scale(&self, scale: Vec3) -> Mat4 {
        *self * Mat4::create_scale(scale)
    }

    pub fn rotate(&self, theta: f64, axis: Vec3) -> Mat4 {
        *self * Mat4::create_rotation(theta, axis)
    }

    // TODO: make sure this is real
    pub fn without_scale(&self) -> Mat4 {
        let mut cells = self.cells.clone();
        cells[0][3] = 0f64;
        cells[1][3] = 0f64;
        cells[2][3] = 0f64;
        Mat4 { cells }
    }

    pub fn transpose(&self) -> Mat4 {
        let mut cells = self.cells.clone();
        for x in 0..4 {
            for y in 0..4 {
                cells[y][x] = self.cells[x][y];
            }
        }
        Mat4 { cells }
    }

    pub fn get_cell(&self, x: usize, y: usize) -> f64 {
        self.cells[y][x]
    }

    // https://stackoverflow.com/questions/1148309/inverting-a-4x4-matrix
    pub fn invert(&self) -> Option<Mat4> {
        let mut inverse = [[0f64; 4]; 4];
        let m = self.cells;

        inverse[0][0] = m[1][1] * m[2][2] * m[3][3] -
            m[1][1] * m[3][2] * m[2][3] -
            m[1][2] * m[2][1] * m[3][3] +
            m[1][2] * m[3][1] * m[2][3] +
            m[1][3] * m[2][1] * m[3][2] -
            m[1][3] * m[3][1] * m[2][2];

        inverse[0][1] = -m[0][1] * m[2][2] * m[3][3] +
            m[0][1] * m[3][2] * m[2][3] +
            m[0][2] * m[2][1] * m[3][3] -
            m[0][2] * m[3][1] * m[2][3] -
            m[0][3] * m[2][1] * m[3][2] +
            m[0][3] * m[3][1] * m[2][2];

        inverse[0][2] = m[0][1] * m[1][2] * m[3][3] -
            m[0][1] * m[3][2] * m[1][3] -
            m[0][2] * m[1][1] * m[3][3] +
            m[0][2] * m[3][1] * m[1][3] +
            m[0][3] * m[1][1] * m[3][2] -
            m[0][3] * m[3][1] * m[1][2];

        inverse[0][3] = -m[0][1] * m[1][2] * m[2][3] +
            m[0][1] * m[2][2] * m[1][3] +
            m[0][2] * m[1][1] * m[2][3] -
            m[0][2] * m[2][1] * m[1][3] -
            m[0][3] * m[1][1] * m[2][2] +
            m[0][3] * m[2][1] * m[1][2];

        inverse[1][0] = -m[1][0] * m[2][2] * m[3][3] +
            m[1][0] * m[3][2] * m[2][3] +
            m[1][2] * m[2][0] * m[3][3] -
            m[1][2] * m[3][0] * m[2][3] -
            m[1][3] * m[2][0] * m[3][2] +
            m[1][3] * m[3][0] * m[2][2];

        inverse[1][1] = m[0][0] * m[2][2] * m[3][3] -
            m[0][0] * m[3][2] * m[2][3] -
            m[0][2] * m[2][0] * m[3][3] +
            m[0][2] * m[3][0] * m[2][3] +
            m[0][3] * m[2][0] * m[3][2] -
            m[0][3] * m[3][0] * m[2][2];

        inverse[1][2] = -m[0][0] * m[1][2] * m[3][3] +
            m[0][0] * m[3][2] * m[1][3] +
            m[0][2] * m[1][0] * m[3][3] -
            m[0][2] * m[3][0] * m[1][3] -
            m[0][3] * m[1][0] * m[3][2] +
            m[0][3] * m[3][0] * m[1][2];

        inverse[1][3] = m[0][0] * m[1][2] * m[2][3] -
            m[0][0] * m[2][2] * m[1][3] -
            m[0][2] * m[1][0] * m[2][3] +
            m[0][2] * m[2][0] * m[1][3] +
            m[0][3] * m[1][0] * m[2][2] -
            m[0][3] * m[2][0] * m[1][2];

        inverse[2][0] = m[1][0] * m[2][1] * m[3][3] -
            m[1][0] * m[3][1] * m[2][3] -
            m[1][1] * m[2][0] * m[3][3] +
            m[1][1] * m[3][0] * m[2][3] +
            m[1][3] * m[2][0] * m[3][1] -
            m[1][3] * m[3][0] * m[2][1];

        inverse[2][1] = -m[0][0] * m[2][1] * m[3][3] +
            m[0][0] * m[3][1] * m[2][3] +
            m[0][1] * m[2][0] * m[3][3] -
            m[0][1] * m[3][0] * m[2][3] -
            m[0][3] * m[2][0] * m[3][1] +
            m[0][3] * m[3][0] * m[2][1];

        inverse[2][2] = m[0][0] * m[1][1] * m[3][3] -
            m[0][0] * m[3][1] * m[1][3] -
            m[0][1] * m[1][0] * m[3][3] +
            m[0][1] * m[3][0] * m[1][3] +
            m[0][3] * m[1][0] * m[3][1] -
            m[0][3] * m[3][0] * m[1][1];

        inverse[2][3] = -m[0][0] * m[1][1] * m[2][3] +
            m[0][0] * m[2][1] * m[1][3] +
            m[0][1] * m[1][0] * m[2][3] -
            m[0][1] * m[2][0] * m[1][3] -
            m[0][3] * m[1][0] * m[2][1] +
            m[0][3] * m[2][0] * m[1][1];

        inverse[3][0] = -m[1][0] * m[2][1] * m[3][2] +
            m[1][0] * m[3][1] * m[2][2] +
            m[1][1] * m[2][0] * m[3][2] -
            m[1][1] * m[3][0] * m[2][2] -
            m[1][2] * m[2][0] * m[3][1] +
            m[1][2] * m[3][0] * m[2][1];

        inverse[3][1] = m[0][0] * m[2][1] * m[3][2] -
            m[0][0] * m[3][1] * m[2][2] -
            m[0][1] * m[2][0] * m[3][2] +
            m[0][1] * m[3][0] * m[2][2] +
            m[0][2] * m[2][0] * m[3][1] -
            m[0][2] * m[3][0] * m[2][1];

        inverse[3][2] = -m[0][0] * m[1][1] * m[3][2] +
            m[0][0] * m[3][1] * m[1][2] +
            m[0][1] * m[1][0] * m[3][2] -
            m[0][1] * m[3][0] * m[1][2] -
            m[0][2] * m[1][0] * m[3][1] +
            m[0][2] * m[3][0] * m[1][1];

        inverse[3][3] = m[0][0] * m[1][1] * m[2][2] -
            m[0][0] * m[2][1] * m[1][2] -
            m[0][1] * m[1][0] * m[2][2] +
            m[0][1] * m[2][0] * m[1][2] +
            m[0][2] * m[1][0] * m[2][1] -
            m[0][2] * m[2][0] * m[1][1];

        let determinant = m[0][0] * inverse[0][0] + m[1][0] * inverse[0][1] + m[2][0] * inverse[0][2] + m[3][0] * inverse[0][3];

        if determinant == 0f64 {
            None
        } else {
            for x in 0..4 {
                for y in 0..4 {
                    inverse[y][x] = inverse[y][x] / determinant;
                }
            }

            Some(Mat4 {
                cells: inverse
            })
        }
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut cells = [[0f64; 4]; 4];
        for x in 0..4 {
            for y in 0..4 {
                for i in 0..4 {
                    cells[y][x] += self.get_cell(i, y) * other.get_cell(x, i);
                }
            }
        }

        Mat4 { cells }
    }
}
