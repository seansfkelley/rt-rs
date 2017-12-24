use math::*;
use material::*;
use super::uv::*;

#[derive(Debug, Clone)]
pub struct Intersection {
    pub distance: f64,
    pub location: Point,
    pub normal: Normal,
    pub shading_normal: Option<Normal>,
    pub uv: Option<Uv>,
    pub material: Option<Material>,
}

impl Intersection {
    pub fn with_material(self, material: Material) -> Intersection {
        Intersection {
            distance: self.distance,
            location: self.location,
            normal: self.normal,
            shading_normal: self.shading_normal,
            uv: self.uv,
            material: Some(material),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_add_uv() {
        let sum = Uv(1f64, 2f64) + Uv(10f64, 20f64);
        assert_eq!(sum.0, 11f64);
        assert_eq!(sum.1, 22f64);
    }

    #[test]
    fn it_should_multiply_uv() {
        let product = Uv(1f64, 2f64) * 10f64;
        assert_eq!(product.0, 10f64);
        assert_eq!(product.1, 20f64);
    }
}
