#![allow(dead_code)]
use std::fmt::{ Display, Debug, Formatter, Result };
use std::ops::{ Add, Sub, Div, Mul, Neg, Index, IndexMut };

#[derive(Debug, Clone, Copy)]
pub enum Axis {
    X,
    Y,
    Z,
}

pub trait Xyz {
    fn x(&self) -> f64;
    fn y(&self) -> f64;
    fn z(&self) -> f64;
}

macro_rules! foreach_axis {
    ($i:ident in $body:block) => {
        let $i = Axis::X;
        $body
        let $i = Axis::Y;
        $body
        let $i = Axis::Z;
        $body
    }
}

macro_rules! xyz_base {
    ($name:ident) => {
        #[derive(PartialEq, Clone, Copy)]
        pub struct $name {
            pub x: f64,
            pub y: f64,
            pub z: f64,
        }

        impl $name {
            pub fn new(x: f64, y: f64, z: f64) -> $name {
                assert!(!x.is_nan() && !y.is_nan() && !z.is_nan(), "had NaN terms in {} construction", stringify!($name));
                $name { x, y, z }
            }

            pub fn uniform(value: f64) -> $name {
                assert!(!value.is_nan(), "had NaN terms in {} construction", stringify!($name));
                $name { x: value, y: value, z: value }
            }

            fn format(&self, f: &mut Formatter) -> Result {
                match f.precision() {
                    Some(p) => {
                        write!(f, "{}<{:.*}, {:.*}, {:.*}>", stringify!($name), p, self.x, p, self.y, p, self.z)
                    }
                    None => {
                        write!(f, "{}<{}, {}, {}>", stringify!($name), self.x, self.y, self.z)
                    }
                }
            }
        }

        impl Index<Axis> for $name {
            type Output = f64;

            fn index(&self, index: Axis) -> &f64 {
                match index {
                    Axis::X => &self.x,
                    Axis::Y => &self.y,
                    Axis::Z => &self.z,
                }
            }
        }

        impl IndexMut<Axis> for $name {
            fn index_mut(&mut self, index: Axis) -> &mut f64 {
                match index {
                    Axis::X => &mut self.x,
                    Axis::Y => &mut self.y,
                    Axis::Z => &mut self.z,
                }
            }
        }

        impl Display for $name {
            fn fmt(&self, f: &mut Formatter) -> Result {
                self.format(f)
            }
        }

        impl Debug for $name {
            fn fmt(&self, f: &mut Formatter) -> Result {
                self.format(f)
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
                $result::new(
                    self.x $opsymbol other.x,
                    self.y $opsymbol other.y,
                    self.z $opsymbol other.z,
                )
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

pub trait Dottable : Xyz {
    fn dot(&self, other: &Dottable) -> f64;
}

macro_rules! xyz_dot {
    ($name:ident) => {
        impl Dottable for $name {
            fn dot(&self, other: &Dottable) -> f64 {
                self.x * other.x() + self.y * other.y() + self.z * other.z()
            }
        }
    };
}

macro_rules! xyz_cross {
    ($lhs:ident, $rhs:ident, $result:ident) => {
        impl $lhs {
            pub fn cross(&self, other: $rhs) -> $result {
                $result::new(
                    self.y * other.z - self.z * other.y,
                    self.z * other.x - self.x * other.z,
                    self.x * other.y - self.y * other.x,
                )
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

            pub fn into_normalized(self) -> $name {
                let magnitude = self.magnitude();
                self / magnitude
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
    ($name:ident, $result:ident, $as_name:ident, $into_name:ident) => {
        impl $name {
            pub fn $as_name(&self) -> $result {
                $result::new(
                    self.x,
                    self.y,
                    self.z,
                )
            }

            pub fn $into_name(self) -> $result {
                $result::new(
                    self.x,
                    self.y,
                    self.z,
                )
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
xyz_normalizable!(Vec3);
xyz_convertible!(Vec3, Normal, as_normal, into_normal);
xyz_convertible!(Vec3, Point, as_point, into_point);

xyz_base!(Point);
xyz_neg!(Point);
xyz_add!(Point, Point, Point);
xyz_add!(Point, Vec3, Point);
xyz_sub!(Point, Point, Vec3);
xyz_sub!(Point, Vec3, Point);
xyz_add!(Point, Normal, Point);
xyz_sub!(Point, Normal, Point);
xyz_mul!(Point);
xyz_div!(Point);
xyz_dot!(Point);
xyz_convertible!(Point, Vec3, as_vector, into_vector);
xyz_convertible!(Point, Normal, as_normal, into_normal);

xyz_base!(Normal);
xyz_neg!(Normal);
xyz_add!(Normal, Normal, Normal);
xyz_sub!(Normal, Normal, Normal);
xyz_mul!(Normal);
xyz_div!(Normal);
xyz_dot!(Normal);
xyz_cross!(Normal, Vec3, Vec3);
xyz_normalizable!(Normal);
xyz_convertible!(Normal, Vec3, as_vector, into_vector);

impl Vec3 {
    pub const X_AXIS: Vec3 = Vec3 { x: 1f64, y: 0f64, z: 0f64 };
    pub const Y_AXIS: Vec3 = Vec3 { x: 0f64, y: 1f64, z: 0f64 };
    pub const Z_AXIS: Vec3 = Vec3 { x: 0f64, y: 0f64, z: 1f64 };

    pub fn assert_normalized(&self) {
        const EPSILON: f64 = 1e-10;
        let magnitude = (self.magnitude() - 1f64).abs();
        assert!(magnitude < EPSILON, "magnitude {} >= epsilon {}", magnitude, EPSILON);
    }

    pub fn reflect(&self, axis: Vec3) -> Vec3 {
        self.assert_normalized();
        axis.assert_normalized();
        *self - axis * (2f64 * self.dot(&axis))
    }
}
