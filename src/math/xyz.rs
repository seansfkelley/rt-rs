#![allow(dead_code)]
use std::ops::{Add, Sub, Div, Mul, Neg, Index};

pub trait Xyz {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

macro_rules! xyz_base {
    ($name:ident) => {
        #[derive(Debug, Copy, Clone, PartialEq)]
        pub struct $name {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }

        impl $name {
            pub fn new(x: f64, y: f64, z: f64) -> $name {
                $name { x, y, z }
            }

            pub fn uniform(value: f64) -> $name {
                $name { x: value, y: value, z: value }
            }
        }

        impl Index<usize> for $name {
            type Output = f64;

            fn index(&self, index: usize) -> &f64 {
                match index {
                    0 => &self.x,
                    1 => &self.y,
                    2 => &self.z,
                    _ => { panic!("index out of range: {}", index); }
                }
            }
        }

        impl Xyz for $name {
            fn x(&self) -> f64 { self.x }
            fn y(&self) -> f64 { self.y }
            fn z(&self) -> f64 { self.z }
        }
    };
}

macro_rules! xyz_op_xyz {
    ($opname:ident, $fnname:ident, $opsymbol:tt, $lhs:ident, $rhs:ident, $result:ident) => {
        impl $opname<$rhs> for $lhs {
            type Output = $result;

            fn $fnname(self, other: $rhs) -> $result {
                $result {
                    x: self.x $opsymbol other.x,
                    y: self.y $opsymbol other.y,
                    z: self.z $opsymbol other.z,
                }
            }
        }
    };
}

macro_rules! xyz_op_f64 {
    ($opname:ident, $fnname:ident, $opsymbol:tt, $xyz_kind:ident) => {
        impl $opname<f64> for $xyz_kind {
            type Output = $xyz_kind;

            fn $fnname(self, other: f64) -> $xyz_kind {
                $xyz_kind {
                    x: self.x $opsymbol other,
                    y: self.y $opsymbol other,
                    z: self.z $opsymbol other,
                }
            }
        }
    };
}

macro_rules! xyz_neg {
    ($name:ident) => {
        impl Neg for $name {
            type Output = $name;

            fn neg(self) -> $name {
                $name {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                }
            }
        }
    };
}

macro_rules! xyz_add {
    ($lhs:ident, $rhs:ident, $result:ident) => {
        xyz_op_xyz!(Add, add, +, $lhs, $rhs, $result);
    };
}

macro_rules! xyz_sub {
    ($lhs:ident, $rhs:ident, $result:ident) => {
        xyz_op_xyz!(Sub, sub, -, $lhs, $rhs, $result);
    };
}

macro_rules! xyz_mul {
    ($name:ident) => {
        xyz_op_f64!(Mul, mul, *, $name);
    };
}

macro_rules! xyz_div {
    ($name:ident) => {
        xyz_op_f64!(Div, div, /, $name);
    };
}

pub trait Dottable {
    fn dot(&self, other: &Xyz) -> f64;
}

macro_rules! xyz_dot {
    ($name:ident) => {
        impl Dottable for $name {
            fn dot(&self, other: &Xyz) -> f64 {
                self.x * other.x() + self.y * other.y() + self.z * other.z()
            }
        }
    };
}

macro_rules! xyz_cross {
    ($lhs:ident, $rhs:ident, $result:ident) => {
        impl $lhs {
            pub fn cross(&self, other: $rhs) -> $result {
                $result {
                    x: self.y * other.z - self.z * other.y,
                    y: self.z * other.x - self.x * other.z,
                    z: self.x * other.y - self.y * other.x,
                }
            }
        }
    };
}

macro_rules! xyz_normalizable {
    ($name:ident) => {
        impl $name {
            pub fn as_normalized(&self) -> $name {
                *self / self.magnitude()
            }

            pub fn magnitude2(&self) -> f64 {
                self.dot(self)
            }

            pub fn magnitude(&self) -> f64 {
                self.magnitude2().sqrt()
            }

        }
    };
}

macro_rules! xyz_convertible {
    ($name:ident, $result:ident, $fnname: ident) => {
        impl $name {
            pub fn $fnname(&self) -> $result {
                $result {
                    x: self.x,
                    y: self.y,
                    z: self.z,
                }
            }
        }
    };
}

xyz_base!(Vec3);
xyz_neg!(Vec3);
xyz_add!(Vec3, Vec3, Vec3);
xyz_sub!(Vec3, Vec3, Vec3);
xyz_mul!(Vec3);
xyz_div!(Vec3);
xyz_dot!(Vec3);
xyz_cross!(Vec3, Vec3, Vec3);
// TODO: Function overloading!
// xyz_cross!(Vec3, Normal, Vec3);
xyz_normalizable!(Vec3);
xyz_convertible!(Vec3, Normal, to_normal);

xyz_base!(Point);
xyz_neg!(Point);
xyz_add!(Point, Point, Point);
xyz_add!(Point, Vec3, Point);
xyz_sub!(Point, Point, Vec3);
xyz_sub!(Point, Vec3, Point);
xyz_mul!(Point);
xyz_div!(Point);
xyz_convertible!(Point, Vec3, to_vector);

xyz_base!(Normal);
xyz_neg!(Normal);
xyz_add!(Normal, Normal, Normal);
xyz_sub!(Normal, Normal, Normal);
xyz_mul!(Normal);
xyz_div!(Normal);
xyz_dot!(Normal);
xyz_cross!(Normal, Vec3, Vec3);
xyz_normalizable!(Normal);
xyz_convertible!(Normal, Vec3, to_vector);

impl Vec3 {
    // TODO: Remove this?
    pub fn assert_normalized(&self) {
        debug_assert!((self.magnitude() - 1f64).abs() < 1e-10);
    }

    pub fn reflect(&self, axis: Vec3) -> Vec3 {
        // If we decide to keep this, should do the normalization ourselves.
        self.assert_normalized();
        *self - axis * (2f64 * self.dot(&axis))
    }
}
