use std::ops::{Add, Sub, Div, Mul, Neg};
use vector::Vec3;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Mat4 {
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

impl Mat4 {
    pub fn create() -> Mat4 {
        Mat4 { cells: [[0f64; 4]; 4] }
    }

//    pub fn rotate(&self, theta: f64) -> Mat4 {
//
//    }

    pub fn get_cell(&self, x: usize, y: usize) -> f64 {
        self.cells[y][x]
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

impl Mul<Vec3> for Mat4 {
    type Output = Vec3;

    fn mul(self, other: Vec3) -> Vec3 {
        let mut vec4 = [0f64; 4];

        for i in 0..4 {
            vec4[i] = self.get_cell(0, i) * other.x
                + self.get_cell(1, i) * other.y
                + self.get_cell(2, i) * other.z
                + self.get_cell(3, i);
        }

        Vec3 {
            x: vec4[0],
            y: vec4[1],
            z: vec4[2],
        }
    }
}
