use core::*;
use math::*;
use color::{Color, BLACK};
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

    pub fn raytrace(&self, ray: &Ray) -> Color {
        self.get_color(ray, 0)
    }

    fn cast_ray(&self, ray: &Ray, depth: u32) -> Option<SceneObjectHit> {
        if depth > self.depth_limit {
            None
        } else {
            let mut closest: Option<SceneObjectHit> = Option::None;

            for o in &self.objects {
                match o.intersect(&ray) {
                    Some(hit) => {
                        if hit.enter.is_some() {
                            let complete_hit = hit.unwrap();
                            if closest.is_some() {
                                if complete_hit.enter.distance < closest.as_ref().unwrap().hit.enter.distance {
                                    closest = Some(SceneObjectHit {
                                        hit: complete_hit,
                                        scene_object: &o,
                                    });
                                }
                            } else {
                                closest = Some(SceneObjectHit {
                                    hit: complete_hit,
                                    scene_object: &o,
                                });
                            }
                        }
                    }
                    None => {}
                }
            }

            closest
        }
    }

    pub fn get_color(&self, ray: &Ray, depth: u32) -> Color {
        match self.cast_ray(ray, depth) {
            Some(object_hit) => object_hit.get_color(ray, self, depth),
            None => self.background_color,
        }
    }

    pub fn get_visible_lights(&self, point: Point) -> Vec<&Light> {
        self.lights
            .iter()
            .filter(|light| {
                let light_direction = (light.position - point).as_normalized();
                let ref ray = Ray::new(point, light_direction);
                self.cast_ray(ray, 0u32).is_none()
            })
            .collect::<Vec<&Light>>()
    }
}

