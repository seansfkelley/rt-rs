use core::*;
use geometry::*;
use math::*;
use core::transform;

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

    pub fn transform(&self, transform: &Transform) -> Camera {
        Camera {
            position: self.position.object_to_world(transform),
            up: self.up.object_to_world(transform),
            direction: self.direction.object_to_world(transform),
            right: self.right.object_to_world(transform),
        }
    }
}
