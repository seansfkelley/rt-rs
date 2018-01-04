use std::sync::Arc;
use math::*;
use super::material::Material;
use super::transform::{ Transform, Transformable };
use super::uv::Uv;

#[derive(Debug)]
pub struct IntersectionGeometry {
    pub normal: Normal,
    pub u_axis: Vec3,
    pub v_axis: Vec3,
}

#[derive(Debug)]
pub struct Intersection {
    pub distance: f64,
    pub location: Point,
    pub geometry: IntersectionGeometry,
    pub shading_geometry: Option<IntersectionGeometry>,
    pub uv: Uv,
    pub material: Option<Arc<Material>>,
}

impl Intersection {
    pub fn with_material(self, material: Arc<Material>) -> Intersection {
        Intersection {
            distance: self.distance,
            location: self.location,
            geometry: self.geometry,
            shading_geometry: self.shading_geometry,
            uv: self.uv,
            material: Some(material),
        }
    }
}

impl Transformable for IntersectionGeometry {
    fn transform(self, transform: &Transform) -> IntersectionGeometry {
        IntersectionGeometry {
            normal: self.normal.transform(transform),
            u_axis: self.u_axis.transform(transform),
            v_axis: self.v_axis.transform(transform),
        }
    }

    fn invert_transform(self, transform: &Transform) -> IntersectionGeometry {
        IntersectionGeometry {
            normal: self.normal.invert_transform(transform),
            u_axis: self.u_axis.invert_transform(transform),
            v_axis: self.v_axis.invert_transform(transform),
        }
    }
}

impl Transformable for Intersection {
    fn transform(self, transform: &Transform) -> Intersection {
        Intersection {
            location: self.location.transform(transform),
            geometry: self.geometry.transform(transform),
            shading_geometry: self.shading_geometry.map(|g| g.transform(transform)),
            ..self
        }
    }

    fn invert_transform(self, transform: &Transform) -> Intersection {
        Intersection {
            location: self.location.invert_transform(transform),
            geometry: self.geometry.invert_transform(transform),
            shading_geometry: self.shading_geometry.map(|g| g.invert_transform(transform)),
            ..self
        }
    }
}
