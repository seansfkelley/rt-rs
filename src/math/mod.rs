mod curve;
mod clamp;
mod fuzzy_eq;
mod mat4;
mod non_nan;
mod samples;
#[macro_use]
mod xyz;

pub use self::curve::*;
pub use self::clamp::*;
pub use self::fuzzy_eq::*;
pub use self::mat4::*;
pub use self::non_nan::*;
pub use self::samples::*;
pub use self::xyz::*;

pub const EPSILON: f64 = 1e-10;
