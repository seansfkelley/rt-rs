use math::*;

#[derive(Debug, Clone, Copy)]
pub struct Camera {
    pub position: Point,
    pub up: Vec3,
    pub direction: Vec3,
    pub right: Vec3,
}

impl Camera {
    pub fn look_at(position: Point, up: Vec3, look_at: Point) -> Camera {
        let direction = (look_at - position).as_normalized();
        Camera {
            position,
            up,
            direction,
            right: direction.cross(up).as_normalized(),
        }
    }
}
