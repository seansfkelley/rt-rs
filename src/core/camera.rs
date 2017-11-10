use core::*;
use geometry::*;
use math::*;

pub struct Camera {
    pub position: Point,
    pub up: Vec3,
    pub direction: Vec3,
    pub right: Vec3,
}

impl Camera {
    pub fn look_at_origin(position: Point, up: Vec3) -> Camera {
        let direction = (-position).as_vector().as_normalized();
        Camera {
            position,
            up,
            direction,
            right: direction.cross(up).as_normalized(),
        }
    }
}
