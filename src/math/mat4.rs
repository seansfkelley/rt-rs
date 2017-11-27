use std::ops::Mul;
use std::result::Result;
use std::fmt::{ Debug, Formatter, Error };
use std::ops::Index;
use super::xyz::*;

#[derive(Clone, Copy, PartialEq)]
pub struct Mat4(pub [[f64; 4]; 4]);

pub const IDENTITY_MATRIX: Mat4 = Mat4([
    [1f64, 0f64, 0f64, 0f64],
    [0f64, 1f64, 0f64, 0f64],
    [0f64, 0f64, 1f64, 0f64],
    [0f64, 0f64, 0f64, 1f64],
]);

pub const X_AXIS: Vec3 = Vec3 { x: 1f64, y: 0f64, z: 0f64 };
pub const Y_AXIS: Vec3 = Vec3 { x: 0f64, y: 1f64, z: 0f64 };
pub const Z_AXIS: Vec3 = Vec3 { x: 0f64, y: 0f64, z: 1f64 };

impl Mat4 {
    pub fn create_translation(translation: Vec3) -> Mat4 {
        let mut cells = IDENTITY_MATRIX.clone();
        cells[0][3] = translation.x();
        cells[1][3] = translation.y();
        cells[2][3] = translation.z();
        Mat4(cells)
    }

    pub fn create_scale(scale: Vec3) -> Mat4 {
        let mut cells = [[0f64; 4]; 4];
        cells[0][0] = scale.x();
        cells[1][1] = scale.y();
        cells[2][2] = scale.z();
        cells[3][3] = 1f64;
        Mat4(cells)
    }

    pub fn create_rotation(theta: f64, a: Vec3) -> Mat4 {
        a.assert_normalized();
        let mut cells = [[0f64; 4]; 4];
        let cos_theta = theta.cos();
        let sin_theta = theta.sin();
        let one_minus_cos_theta = 1f64 - cos_theta;

        cells[0][0] = a.x * a.x + (1f64 - a.x * a.x) * cos_theta;
        cells[0][1] = a.x * a.y * one_minus_cos_theta - a.z * sin_theta;
        cells[0][2] = a.x * a.z * one_minus_cos_theta + a.y * sin_theta;

        cells[1][0] = a.x * a.y * one_minus_cos_theta + a.z * sin_theta;
        cells[1][1] = a.y * a.y + (1f64 - a.y * a.y) * cos_theta;
        cells[1][2] = a.y * a.z * one_minus_cos_theta - a.x * sin_theta;

        cells[2][0] = a.x * a.z * one_minus_cos_theta - a.y * sin_theta;
        cells[2][1] = a.y * a.z * one_minus_cos_theta + a.x * sin_theta;
        cells[2][2] = a.z * a.z + (1f64 - a.z * a.z) * cos_theta;

        cells[3][3] = 1f64;
        Mat4(cells)
    }

    // pbrt pg. 84
    pub fn create_look_at(position: Point, look_at: Point, in_up: Vec3) -> Mat4 {
        let direction = (look_at - position).as_normalized();
        let left = in_up.as_normalized().cross(direction).as_normalized();
        let up = direction.cross(left);

        Mat4([
            [left.x, up.x, direction.x, position.x],
            [left.y, up.y, direction.y, position.y],
            [left.z, up.z, direction.z, position.z],
            [  0f64, 0f64,        0f64,       1f64]
        ]).invert().unwrap()
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

    pub fn look_at(&self, position: Point, look_at: Point, in_up: Vec3) -> Mat4 {
        *self * Mat4::create_look_at(position, look_at, in_up)
    }

    // https://stackoverflow.com/questions/1148309/inverting-a-4x4-matrix
    pub fn invert(&self) -> Option<Mat4> {
        let mut inverse = [[0f64; 4]; 4];
        let m = *self;

        inverse[0][0] =
            m[1][1] * m[2][2] * m[3][3] -
            m[1][1] * m[3][2] * m[2][3] -
            m[1][2] * m[2][1] * m[3][3] +
            m[1][2] * m[3][1] * m[2][3] +
            m[1][3] * m[2][1] * m[3][2] -
            m[1][3] * m[3][1] * m[2][2];

        inverse[0][1] =
           -m[0][1] * m[2][2] * m[3][3] +
            m[0][1] * m[3][2] * m[2][3] +
            m[0][2] * m[2][1] * m[3][3] -
            m[0][2] * m[3][1] * m[2][3] -
            m[0][3] * m[2][1] * m[3][2] +
            m[0][3] * m[3][1] * m[2][2];

        inverse[0][2] =
            m[0][1] * m[1][2] * m[3][3] -
            m[0][1] * m[3][2] * m[1][3] -
            m[0][2] * m[1][1] * m[3][3] +
            m[0][2] * m[3][1] * m[1][3] +
            m[0][3] * m[1][1] * m[3][2] -
            m[0][3] * m[3][1] * m[1][2];

        inverse[0][3] =
           -m[0][1] * m[1][2] * m[2][3] +
            m[0][1] * m[2][2] * m[1][3] +
            m[0][2] * m[1][1] * m[2][3] -
            m[0][2] * m[2][1] * m[1][3] -
            m[0][3] * m[1][1] * m[2][2] +
            m[0][3] * m[2][1] * m[1][2];

        inverse[1][0] =
           -m[1][0] * m[2][2] * m[3][3] +
            m[1][0] * m[3][2] * m[2][3] +
            m[1][2] * m[2][0] * m[3][3] -
            m[1][2] * m[3][0] * m[2][3] -
            m[1][3] * m[2][0] * m[3][2] +
            m[1][3] * m[3][0] * m[2][2];

        inverse[1][1] =
            m[0][0] * m[2][2] * m[3][3] -
            m[0][0] * m[3][2] * m[2][3] -
            m[0][2] * m[2][0] * m[3][3] +
            m[0][2] * m[3][0] * m[2][3] +
            m[0][3] * m[2][0] * m[3][2] -
            m[0][3] * m[3][0] * m[2][2];

        inverse[1][2] =
           -m[0][0] * m[1][2] * m[3][3] +
            m[0][0] * m[3][2] * m[1][3] +
            m[0][2] * m[1][0] * m[3][3] -
            m[0][2] * m[3][0] * m[1][3] -
            m[0][3] * m[1][0] * m[3][2] +
            m[0][3] * m[3][0] * m[1][2];

        inverse[1][3] =
            m[0][0] * m[1][2] * m[2][3] -
            m[0][0] * m[2][2] * m[1][3] -
            m[0][2] * m[1][0] * m[2][3] +
            m[0][2] * m[2][0] * m[1][3] +
            m[0][3] * m[1][0] * m[2][2] -
            m[0][3] * m[2][0] * m[1][2];

        inverse[2][0] =
            m[1][0] * m[2][1] * m[3][3] -
            m[1][0] * m[3][1] * m[2][3] -
            m[1][1] * m[2][0] * m[3][3] +
            m[1][1] * m[3][0] * m[2][3] +
            m[1][3] * m[2][0] * m[3][1] -
            m[1][3] * m[3][0] * m[2][1];

        inverse[2][1] =
           -m[0][0] * m[2][1] * m[3][3] +
            m[0][0] * m[3][1] * m[2][3] +
            m[0][1] * m[2][0] * m[3][3] -
            m[0][1] * m[3][0] * m[2][3] -
            m[0][3] * m[2][0] * m[3][1] +
            m[0][3] * m[3][0] * m[2][1];

        inverse[2][2] =
            m[0][0] * m[1][1] * m[3][3] -
            m[0][0] * m[3][1] * m[1][3] -
            m[0][1] * m[1][0] * m[3][3] +
            m[0][1] * m[3][0] * m[1][3] +
            m[0][3] * m[1][0] * m[3][1] -
            m[0][3] * m[3][0] * m[1][1];

        inverse[2][3] =
           -m[0][0] * m[1][1] * m[2][3] +
            m[0][0] * m[2][1] * m[1][3] +
            m[0][1] * m[1][0] * m[2][3] -
            m[0][1] * m[2][0] * m[1][3] -
            m[0][3] * m[1][0] * m[2][1] +
            m[0][3] * m[2][0] * m[1][1];

        inverse[3][0] =
           -m[1][0] * m[2][1] * m[3][2] +
            m[1][0] * m[3][1] * m[2][2] +
            m[1][1] * m[2][0] * m[3][2] -
            m[1][1] * m[3][0] * m[2][2] -
            m[1][2] * m[2][0] * m[3][1] +
            m[1][2] * m[3][0] * m[2][1];

        inverse[3][1] =
            m[0][0] * m[2][1] * m[3][2] -
            m[0][0] * m[3][1] * m[2][2] -
            m[0][1] * m[2][0] * m[3][2] +
            m[0][1] * m[3][0] * m[2][2] +
            m[0][2] * m[2][0] * m[3][1] -
            m[0][2] * m[3][0] * m[2][1];

        inverse[3][2] =
           -m[0][0] * m[1][1] * m[3][2] +
            m[0][0] * m[3][1] * m[1][2] +
            m[0][1] * m[1][0] * m[3][2] -
            m[0][1] * m[3][0] * m[1][2] -
            m[0][2] * m[1][0] * m[3][1] +
            m[0][2] * m[3][0] * m[1][1];

        inverse[3][3] =
            m[0][0] * m[1][1] * m[2][2] -
            m[0][0] * m[2][1] * m[1][2] -
            m[0][1] * m[1][0] * m[2][2] +
            m[0][1] * m[2][0] * m[1][2] +
            m[0][2] * m[1][0] * m[2][1] -
            m[0][2] * m[2][0] * m[1][1];

        let determinant =
            m[0][0] * inverse[0][0] +
            m[1][0] * inverse[0][1] +
            m[2][0] * inverse[0][2] +
            m[3][0] * inverse[0][3];

        if determinant == 0f64 {
            None
        } else {
            for x in 0..4 {
                for y in 0..4 {
                    inverse[y][x] = inverse[y][x] / determinant;
                }
            }

            Some(Mat4(inverse))
        }
    }
}

impl Mul for Mat4 {
    type Output = Mat4;

    fn mul(self, other: Mat4) -> Mat4 {
        let mut cells = [[0f64; 4]; 4];
        for row in 0..4 {
            for col in 0..4 {
                for i in 0..4 {
                    cells[row][col] += self[row][i] * other[i][col];
                }
            }
        }

        Mat4(cells)
    }
}

impl Debug for Mat4 {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        write!(f, "[ \n")?;
        for i in 0..4 {
            write!(f, "  [ ")?;
            for j in 0..4 {
                write!(f, "{}", (*self)[i][j])?;
                if j != 3 {
                    write!(f, ", ")?;
                }
            }
            write!(f, " ]\n")?;
        }
        write!(f, "]")?;
        Result::Ok(())
    }
}

// impl Index<(usize, usize)> for Mat4 {
//     type Output = f64;

//     fn index(&self, index: (usize, usize)) -> &f64 {
//         self[index.0][index.1]
//     }
// }

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_MATRIX: Mat4 = Mat4([
        [ 1f64,  2f64,  3f64,  4f64],
        [ 5f64,  6f64,  7f64,  8f64],
        [ 9f64, 10f64, 11f64, 12f64],
        [13f64, 14f64, 15f64, 16f64],
    ]);

    mod multiplication {
        use super::*;

        #[test]
        fn it_should_not_do_anything_when_multiplying_by_the_identity_matrix() {
            assert_eq!(TEST_MATRIX * IDENTITY_MATRIX, TEST_MATRIX);
        }

        #[test]
        fn it_should_multiply_matrices() {
            let expected = Mat4([
                [90f64,  100f64, 110f64, 120f64],
                [202f64, 228f64, 254f64, 280f64],
                [314f64, 356f64, 398f64, 440f64],
                [426f64, 484f64, 542f64, 600f64]
            ]);
            assert_eq!(TEST_MATRIX * TEST_MATRIX, expected);
        }
    }

    mod inversion {
        use super::*;

        #[test]
        fn it_should_be_none_for_uninvertible_matrices() {
            assert_eq!(TEST_MATRIX.invert(), None);
        }

        #[test]
        fn it_should_invert_identity_to_identity() {
            assert_eq!(IDENTITY_MATRIX.invert(), Some(IDENTITY_MATRIX));
        }

        #[test]
        fn it_should_invert_an_invertible_matrix() {
            let invertible_matrix = Mat4([
                [2f64,  4f64,   8f64,  16f64],
                [3f64,  9f64,  27f64,  81f64],
                [4f64, 16f64,  64f64, 256f64],
                [5f64, 25f64, 125f64, 625f64],
            ]);
            let expected: Mat4 = Mat4([
                [ 5f64,          -20f64 / 3f64,  15f64 / 4f64, -4f64 / 5f64],
                [-47f64 / 12f64,  19f64 / 3f64, -31f64 / 8f64,  13f64 / 15f64],
                [ 1f64,          -11f64 / 6f64,  5f64 / 4f64,  -3f64 / 10f64],
                [-1f64 / 12f64,   1f64 / 6f64,  -1f64 / 8f64,   1f64 / 30f64],
            ]);
            assert_eq!(invertible_matrix.invert(), Some(expected));
        }
    }

    mod transforms {
        use super::*;
        use std::f64::consts::PI;

        #[test]
        fn it_should_create_a_translation_matrix() {
            let expected = Mat4([
                [1f64, 0f64, 0f64, 10f64],
                [0f64, 1f64, 0f64, 20f64],
                [0f64, 0f64, 1f64, 30f64],
                [0f64, 0f64, 0f64,  1f64],
            ]);
            assert_eq!(Mat4::create_translation(Vec3::new(10f64, 20f64, 30f64)), expected);
        }

        #[test]
        fn it_should_create_a_scale_matrix() {
            let expected = Mat4([
                [10f64,  0f64,  0f64, 0f64],
                [0f64,  20f64,  0f64, 0f64],
                [0f64,   0f64, 30f64, 0f64],
                [0f64,   0f64,  0f64, 1f64],
            ]);
            assert_eq!(Mat4::create_scale(Vec3::new(10f64, 20f64, 30f64)), expected);
        }

        #[test]
        fn it_should_create_a_rotation_matrix_around_x() {
            let theta = PI / 4f64;
            let expected = Mat4([
                [1f64,        0f64,         0f64, 0f64],
                [0f64, theta.cos(), -theta.sin(), 0f64],
                [0f64, theta.sin(),  theta.cos(), 0f64],
                [0f64,        0f64,         0f64, 1f64],
            ]);
            assert_eq!(Mat4::create_rotation(theta, X_AXIS), expected);
        }

        #[test]
        fn it_should_create_a_rotation_matrix_around_y() {
            let theta = PI / 4f64;
            let expected = Mat4([
                [ theta.cos(), 0f64, theta.sin(), 0f64],
                [        0f64, 1f64,        0f64, 0f64],
                [-theta.sin(), 0f64, theta.cos(), 0f64],
                [        0f64, 0f64,        0f64, 1f64],
            ]);
            assert_eq!(Mat4::create_rotation(theta, Y_AXIS), expected);
        }

        #[test]
        fn it_should_create_a_rotation_matrix_around_z() {
            let theta = PI / 4f64;
            let expected = Mat4([
                [theta.cos(), -theta.sin(), 0f64, 0f64],
                [theta.sin(),  theta.cos(), 0f64, 0f64],
                [       0f64,         0f64, 1f64, 0f64],
                [       0f64,         0f64, 0f64, 1f64],
            ]);
            assert_eq!(Mat4::create_rotation(theta, Z_AXIS), expected);
        }

        #[test]
        fn it_should_create_a_look_at_matrix() {
            // TODO
        }
    }
}
