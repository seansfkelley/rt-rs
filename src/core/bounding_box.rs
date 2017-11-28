use std::f64;
use math::*;
use super::ray::Ray;
use super::transform::{ Transform, Transformable };

macro_rules! swap {
    ($a:ident, $b:ident) => {
        let temp = $a;
        $a = $b;
        $b = temp;
    };
}

#[derive(Debug, Clone)]
pub struct BoundingBox {
    pub min: Point,
    pub max: Point,
}

impl BoundingBox {
    pub fn empty() -> BoundingBox {
        BoundingBox {
            min: Point::uniform(f64::INFINITY),
            max: Point::uniform(f64::NEG_INFINITY),
        }
    }

    pub fn union(bb1: &BoundingBox, bb2: &BoundingBox) -> BoundingBox {
        BoundingBox {
            min: Point::new(
                non_nan_min(bb1.min.x, bb2.min.x),
                non_nan_min(bb1.min.y, bb2.min.y),
                non_nan_min(bb1.min.z, bb2.min.z),
            ),
            max: Point::new(
                non_nan_max(bb1.max.x, bb2.max.x),
                non_nan_max(bb1.max.y, bb2.max.y),
                non_nan_max(bb1.max.z, bb2.max.z),
            ),
        }
    }

    pub fn with_point(&self, p: &Point) -> BoundingBox {
        BoundingBox {
            min: Point::new(
                non_nan_min(self.min.x, p.x),
                non_nan_min(self.min.y, p.y),
                non_nan_min(self.min.z, p.z),
            ),
            max: Point::new(
                non_nan_max(self.max.x, p.x),
                non_nan_max(self.max.y, p.y),
                non_nan_max(self.max.z, p.z),
            ),
        }
    }

    // pbrt pg. 194
    pub fn intersect(&self, ray: &Ray) -> bool {
        let (mut t0, mut t1) = (0f64, f64::INFINITY);

        for i in 0..3 {
            let (mut t_near, mut t_far) = (
                (self.min[i] - ray.origin[i]) / ray.direction[i],
                (self.max[i] - ray.origin[i]) / ray.direction[i],
            );
            if t_near > t_far {
                swap!(t_near, t_far);
            }
            t0 = non_nan_max(t0, t_near);
            t1 = non_nan_min(t1, t_far);
            if t0 > t1 {
                return false;
            }
        }

        true
    }
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
                let c = candidates[i];
                for j in 0..3 {
                    if c[j] < min[j] {
                        min[j] = c[j];
                    }
                    if c[j] > max[j] {
                        max[j] = c[j];
                    }
                }
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
    fn it_should_do_nothing_when_merged_with_a_point_inside() {
        let bb = BoundingBox {
            min: Point::uniform(-1f64),
            max: Point::uniform(1f64),
        }.with_point(
            Point::uniform(0f64),
        );
        assert_eq!(bb.min, Point::uniform(-1f64));
        assert_eq!(bb.max, Point::uniform(1f64));
    }

    #[test]
    fn it_should_expand_the_box_when_merged_with_a_point_outside() {
        let bb = BoundingBox {
            min: Point::uniform(-1f64),
            max: Point::uniform(1f64),
        }.with_point(
            Point::new(2f64, -3f64, 4f64),
        );
        assert_eq!(bb.min, Point::new(-1f64, -3f64, -1f64));
        assert_eq!(bb.max, Point::new(2f64, 1f64, 4f64));
    }

    #[test]
    fn it_should_maintain_min_max_under_transformation() {
        let bb = BoundingBox {
            min: Point::uniform(-1f64),
            max: Point::uniform(2f64),
        }.transform(
            &Transform::new(Mat4::create_scale(Vec3::uniform(-3f64)))
        );
        assert_eq!(bb.min, Point::uniform(-6f64));
        assert_eq!(bb.max, Point::uniform(3f64));
    }

    #[test]
    fn it_should_bloat_the_bounding_box_under_some_transformations() {
        let bb = BoundingBox {
            min: Point::uniform(-1f64),
            max: Point::uniform(1f64),
        }.transform(
            &Transform::new(Mat4::create_rotation(45f64.to_radians(), Vec3::Z_AXIS))
        );
        assert_near!(bb.min, Point::new(-2f64.sqrt(), -2f64.sqrt(), -1f64));
        assert_near!(bb.max, Point::new( 2f64.sqrt(),  2f64.sqrt(),  1f64));
    }
}
