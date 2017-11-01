use vector::Vec3;
use color::Color;
use material::Material;
use util::Clamp;
use std::f64::consts::PI;
use std::rc::Rc;

pub type Uv = (f64, f64);

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Ray {
        assert!((direction.magnitude() - 1f64).abs() < 1e-10);
        Ray { origin, direction }
    }

    pub fn at(&self, distance: f64) -> Vec3 {
        self.origin + self.direction * distance
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Light {
    pub position: Vec3,
    pub color: Color,
}

impl Light {
    pub fn new(position: Vec3, color: Color) -> Light {
        Light { position, color }
    }
}

pub struct Intersection {
    pub distance: f64,
    pub location: Vec3,
    pub normal: Vec3,
    pub uv: Uv,
}

pub struct Hit<'a> {
    pub enter: Option<Intersection>,
    pub exit: Intersection,
    pub object: &'a (SceneObject + 'a),
    pub debug: bool,
}

impl<'a> Hit<'a> {
    pub fn debug(self, debug: bool) -> Hit<'a> {
        Hit {
            enter: self.enter,
            exit: self.exit,
            object: self.object,
            debug,
        }
    }
}

pub trait SceneObject {
    fn intersect(&self, ray: &Ray) -> Option<Hit>;
    fn material(&self) -> Rc<Material>;
}

#[derive(Debug)]
pub struct Sphere {
    center: Vec3,
    radius: f64,
    material: Rc<Material>,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: &Rc<Material>) -> Sphere {
        Sphere { center, radius, material: Rc::clone(material) }
    }

    fn get_intersection(&self, t: f64, ray: &Ray) -> Intersection {
        let location = ray.at(t);

        // pbrt pg. 119
        // Make sure we transform into object space!
        let mut phi = (location.y - self.center.y).atan2(location.x - self.center.x);
        if phi < 0f64 {
            phi += 2f64 * PI;
        }
        let theta = ((location.z - self.center.z) / self.radius).clamp(-1f64, 1f64).acos();

        Intersection {
            distance: t,
            location,
            normal: (location - self.center).as_unit_vector(),
            uv: (phi / (2f64 * PI), theta / PI),
        }
    }
}

impl SceneObject for Sphere {
    // TODO: Verify this implementation against pbrt.
    // TODO: Should transform ray into world space first so the rest of the math is easy.
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let l = self.center - ray.origin;
        let t_center = l.dot(ray.direction);
        if t_center + self.radius <= 0f64 {
            None
        } else {
            let d_sq = l.magnitude2() - t_center * t_center;
            let r_sq = self.radius * self.radius; // could cache?
            if d_sq > r_sq {
                None
            } else {
                let t_distance = (r_sq - d_sq).sqrt();
                let t0 = t_center - t_distance;
                let t1 = t_center + t_distance;
                let exit = self.get_intersection(t1, ray);
                if t0 <= 0f64 {
                    Some(Hit {
                        enter: None,
                        exit,
                        object: self,
                        debug: false,
                    })
                } else {
                    Some(Hit {
                        enter: Some(self.get_intersection(t0, ray)),
                        exit,
                        object: self,
                        debug: false,
                    })
                }
            }
        }
    }

    fn material(&self) -> Rc<Material> { Rc::clone(&self.material) }
}

pub struct SubtractedSceneObject {
    lhs: Rc<SceneObject>,
    rhs: Rc<SceneObject>,
}

impl SceneObject for SubtractedSceneObject {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        let lhs_hit_option = self.lhs.intersect(ray);

        match lhs_hit_option {
            Some(lhs_hit) => {
                let rhs_hit_option = self.rhs.intersect(ray);
                match rhs_hit_option {
                    Some(rhs_hit) => {
                        let lhs_enter = lhs_hit.enter.as_ref().map(|enter| enter.distance).unwrap_or(0f64);
                        let rhs_enter = rhs_hit.enter.map(|enter| enter.distance).unwrap_or(0f64);
                        let lhs_exit = lhs_hit.exit.distance;
                        let rhs_exit = rhs_hit.exit.distance;
                        if lhs_enter < rhs_enter || lhs_enter > rhs_exit {
                            Some(lhs_hit)
                        } else {
                            // lhs_enter is inside rhs
                            if rhs_exit < lhs_exit {
                                // Exists from rhs first
                                Some(Hit {
                                    enter: Some(Intersection {
                                        distance: rhs_hit.exit.distance,
                                        location: rhs_hit.exit.location,
                                        normal: -rhs_hit.exit.normal,
                                        uv: rhs_hit.exit.uv,
                                    }),
                                    exit: lhs_hit.exit,
                                    object: lhs_hit.object,
                                    debug: false,
                                })
                            } else {
                                None
                            }
                        }
                    },
                    None => Some(lhs_hit),
                }
            },
            None => None,
        }
    }

    fn material(&self) -> Rc<Material> {
        self.lhs.material()
    }
}

pub fn subtract_scene_objects(lhs: Rc<SceneObject>, rhs: Rc<SceneObject>) -> SubtractedSceneObject {
    SubtractedSceneObject { lhs, rhs }
}

pub type TriangleIndices = (usize, usize, usize);

pub struct TriangleMesh {
    positions: Vec<Vec3>,
    normals: Vec<Vec3>,
    uvs: Vec<Uv>,
    indices: Vec<TriangleIndices>,
    material: Rc<Material>,
}

impl TriangleMesh {
    // FYI, the "front" is when the vertices are in counterclockwise order.
    // > By default, counterclockwise polygons are taken to be front-facing.
    // (https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/glFrontFace.xml)
    pub fn new(positions: Vec<Vec3>, normals: Vec<Vec3>, uvs: Vec<Uv>, indices: Vec<TriangleIndices>, material: &Rc<Material>) -> TriangleMesh {
        // TODO: When we actually do UVs and given normals, do this.
        // assert_eq!(positions.len(), normals.len());
        // assert_eq!(positions.len(), uvs.len());
        // TODO: Also check that the coordinates are in-bounds.
        TriangleMesh { positions, normals, uvs, indices, material: Rc::clone(material) }
    }

    fn intersect_triplet(&self, triplet: &TriangleIndices, ray: &Ray) -> Option<Hit> {
        // pbrt pg. 141
        let &(i1, i2, i3) = triplet;
        let (p1, p2, p3) = (self.positions[i1], self.positions[i2], self.positions[i3]);
        let e1 = p2 - p1;
        let e2 = p3 - p1;
        let s1 = ray.direction.cross(e2);
        let divisor = s1.dot(e1);
        if divisor == 0f64 {
            return None;
        }

        let d = ray.origin - p1;
        let b1 = d.dot(s1) / divisor;
        if b1 < 0f64 || b1 > 1f64 {
            return None;
        }

        let s2 = d.cross(e1);
        let b2 = ray.direction.dot(s2) / divisor;
        if b2 < 0f64 || b1 + b2 > 1f64 {
            return None;
        }

        let t = e2.dot(s2) / divisor;
        if t < 0f64 {
            return None;
        }

        return Some(Hit {
            enter: Some(Intersection {
                distance: t,
                location: ray.at(t),
                normal: e1.cross(e2).as_unit_vector(),
                uv: (0f64, 0f64),
            }),
            exit: Intersection {
                distance: t,
                location: ray.at(t),
                normal: e1.cross(e2).as_unit_vector(),
                uv: (0f64, 0f64),
            },
            object: self,
            debug: false,
        });
    }
}

impl SceneObject for TriangleMesh {
    fn intersect(&self, ray: &Ray) -> Option<Hit> {
        for triplet in &self.indices {
            match self.intersect_triplet(&triplet, ray) {
                Some(hit) => { return Some(hit); }
                None => {}
            }
        }

        return None;
    }

    fn material(&self) -> Rc<Material> { Rc::clone(&self.material) }
}
