use std::cmp::{ min, max };
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
}

// Note that we use `new`, which maintains axis-alignedness, so multiple transforms
// may bloat the size of the box.
impl Transformable for BoundingBox {
    fn transform(&self, transform: &Transform) -> BoundingBox {
        BoundingBox::new(
            self.min.transform(transform),
            self.max.transform(transform),
        )
    }

    fn invert_transform(&self, transform: &Transform) -> BoundingBox {
        BoundingBox::new(
            self.min.invert_transform(transform),
            self.max.invert_transform(transform),
        )
    }
}
