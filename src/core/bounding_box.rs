use std::cmp::{ min, max };
use std::f64;
use math::*;
use super::transform::{ Transform, Transformable };

#[derive(Debug)]
struct BoundingBox {
    min: Point,
    max: Point,
}

impl BoundingBox {
    pub fn new(p1: Point, p2: Point) -> BoundingBox {
        let (p1x, p1y, p1z) = p1.as_notnan();
        let (p2x, p2y, p2z) = p2.as_notnan();
        BoundingBox {
            min: Point::new(
                min(p1x, p2x).into_inner(),
                min(p1x, p2x).into_inner(),
                min(p1y, p2y).into_inner(),
            ),
            max: Point::new(
                max(p1y, p2y).into_inner(),
                max(p1z, p2z).into_inner(),
                max(p1z, p2z).into_inner(),
            ),
        }
    }

    pub fn union(&self, p: Point) -> BoundingBox {
        let (min_x, min_y, min_z) = self.min.as_notnan();
        let (max_x, max_y, max_z) = self.max.as_notnan();
        let (px, py, pz) = p.as_notnan();
        BoundingBox {
            min: Point::new(
                min(min_x, px).into_inner(),
                min(min_y, py).into_inner(),
                min(min_z, pz).into_inner(),
            ),
            max: Point::new(
                max(max_x, px).into_inner(),
                max(max_y, py).into_inner(),
                max(max_z, pz).into_inner(),
            ),
        }
    }
}

macro_rules! compare_and_set {
    ($candidate:expr, $min:ident, $max:ident, $component:ident) => {
        if $candidate.$component < $min.$component {
            $min.$component = $candidate.$component;
        }
        if $candidate.$component > $max.$component {
            $max.$component = $candidate.$component;
        }
    };
}

macro_rules! apply_transform {
    ($self:ident, $fnname:ident, $transform:ident) => {
        {
            let candidates = [
                Point::new($self.min.x, $self.min.y, $self.min.z).$fnname($transform),
                Point::new($self.min.x, $self.min.y, $self.max.z).$fnname($transform),
                Point::new($self.min.x, $self.max.y, $self.min.z).$fnname($transform),
                Point::new($self.min.x, $self.max.y, $self.max.z).$fnname($transform),
                Point::new($self.max.x, $self.min.y, $self.min.z).$fnname($transform),
                Point::new($self.max.x, $self.min.y, $self.max.z).$fnname($transform),
                Point::new($self.max.x, $self.max.y, $self.min.z).$fnname($transform),
                Point::new($self.max.x, $self.max.y, $self.max.z).$fnname($transform),
            ];
            let mut min = Point::uniform(f64::INFINITY);
            let mut max = Point::uniform(f64::NEG_INFINITY);
            for i in 0..8 {
                compare_and_set!(candidates[i], min, max, x);
                compare_and_set!(candidates[i], min, max, y);
                compare_and_set!(candidates[i], min, max, z);
            }
            BoundingBox { min, max }
        }
    };
}

// TODO: Use Arvo 1990 "Transforming Axis-Aligned Bounding Boxes", per pbrt's suggestion.
impl Transformable for BoundingBox {
    fn transform(&self, transform: &Transform) -> BoundingBox {
        apply_transform!(self, transform, transform)
    }

    fn invert_transform(&self, transform: &Transform) -> BoundingBox {
        apply_transform!(self, invert_transform, transform)
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! assert_near {
        ( $ left : expr , $ right : expr ) => {
            assert!(($left - $right).magnitude() < 1e-10f64);
        };
        ($ left : expr , $ right : expr , $ ( $ arg : tt ) + ) => {
            assert!(($left - $right).magnitude() < 1e-10f64, $arg);
        };
    }

    #[test]
    fn it_should_rearrange_values_into_min_and_max_points() {
        let bb = BoundingBox::new(
            Point::new(-1f64, 1f64, -1f64),
            Point::new(1f64, -1f64, 1f64),
        );
        assert_eq!(bb.min, Point::uniform(-1f64));
        assert_eq!(bb.max, Point::uniform(1f64));
    }

    #[test]
    fn it_should_do_nothing_when_unioned_with_a_point_inside() {
        let bb = BoundingBox::new(
            Point::uniform(-1f64),
            Point::uniform(1f64),
        ).union(
            Point::uniform(0f64),
        );
        assert_eq!(bb.min, Point::uniform(-1f64));
        assert_eq!(bb.max, Point::uniform(1f64));
    }

    #[test]
    fn it_should_expand_the_box_when_unioned_with_a_point_outside() {
        let bb = BoundingBox::new(
            Point::uniform(-1f64),
            Point::uniform(1f64),
        ).union(
            Point::new(2f64, -3f64, 4f64),
        );
        assert_eq!(bb.min, Point::new(-1f64, -3f64, -1f64));
        assert_eq!(bb.max, Point::new(2f64, 1f64, 4f64));
    }

    #[test]
    fn it_should_maintain_min_max_under_transformation() {
        let bb = BoundingBox::new(
            Point::uniform(-1f64),
            Point::uniform(2f64),
        ).transform(
            &Transform::new(Mat4::create_scale(Vec3::uniform(-3f64)))
        );
        assert_eq!(bb.min, Point::uniform(-6f64));
        assert_eq!(bb.max, Point::uniform(3f64));
    }

    #[test]
    fn it_should_bloat_the_bounding_box_under_some_transformations() {
        let bb = BoundingBox::new(
            Point::uniform(-1f64),
            Point::uniform(1f64),
        ).transform(
            &Transform::new(Mat4::create_rotation(45f64.to_radians(), Vec3::Z_AXIS))
        );
        assert_near!(bb.min, Point::new(-2f64.sqrt(), -2f64.sqrt(), -1f64));
        assert_near!(bb.max, Point::new( 2f64.sqrt(),  2f64.sqrt(),  1f64));
    }
}
