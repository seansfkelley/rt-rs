use core::*;
use color::Color;
use geometry::Geometry;
use std::rc::Rc;

pub struct Scene {
    objects: Vec<SceneObject>,
    lights: Vec<Light>,
    background_color: Color,
    depth_limit: u32,
}

impl Scene {
    pub fn new(
        objects: Vec<SceneObject>,
        lights: Vec<Light>,
        background_color: Color,
        depth_limit: u32
    ) -> Scene {
        Scene { objects, lights, background_color, depth_limit }
    }

    pub fn raytrace(&self, ray: Ray) -> Color {
        self.get_color(&ray, 0)
    }

    fn cast_ray(&self, ray: &Ray, depth: u32) -> Option<MaterialHit> {
        if depth > self.depth_limit {
            None
        } else {
            let mut closest: Option<MaterialHit> = Option::None;

            for o in &self.objects {
                match o.intersect(&ray) {
                    Some(hit) => {
                        if hit.enter.is_some() {
                            if closest.is_some() {
                                if hit.enter.as_ref().unwrap().distance < closest.as_ref().unwrap().hit.enter.as_ref().unwrap().distance {
                                    closest = Some(MaterialHit { hit, material: Rc::clone(&o.material) });
                                }
                            } else {
                                closest = Some(MaterialHit { hit, material: Rc::clone(&o.material) });
                            }
                        }
                    },
                    None => {},
                }
            }

            closest
        }
    }

    pub fn get_color(&self, ray: &Ray, depth: u32) -> Color {
        match self.cast_ray(ray, depth) {
            Some(material_hit) => {
                let hit = material_hit.hit;
                let material = material_hit.material;
                let intersection = hit.enter.as_ref().unwrap();
                material.get_color(ray, intersection, self, depth)
            },
            None => self.background_color
        }
    }

    pub fn get_visible_lights(&self, intersection: &Intersection) -> Vec<&Light> {
        self.lights
            .iter()
            .filter(|light| {

                let light_direction = (light.position - intersection.location).as_normalized();
                let ref ray = Ray::new(intersection.location, light_direction);
                self.cast_ray(ray, 0u32).map(|hit| hit.hit.enter).is_none()
            })
            .collect::<Vec<&Light>>()
    }
}

