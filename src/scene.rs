use objects;
use color::Color;
use std::f64::consts::PI;

pub struct Scene<'a> {
    objects: Vec<&'a objects::Intersectable>,
    lights: Vec<&'a objects::Light>,
    background_color: Color,
    depth_limit: u32,
}

impl<'a> Scene<'a> {
    pub fn new(
        objects: Vec<&'a objects::Intersectable>,
        lights: Vec<&'a objects::Light>,
        background_color: Color,
        depth_limit: u32
    ) -> Scene<'a> {
        Scene { objects, lights, background_color, depth_limit }
    }

    pub fn raytrace(&self, ray: objects::Ray) -> Color {
        self.raytrace_limited(ray, 0)
    }

    pub fn raytrace_limited(&self, ray: objects::Ray, depth: u32) -> Color {
        if depth > self.depth_limit {
            self.background_color
        } else {
            match self.cast_ray(ray) {
                Some(intersection) => {
                    // TODO: Phong.
                    let phong = intersection.material.ambient;
                    if intersection.material.reflectivity > 0f64 {
                        let new_origin = ray.at(intersection.distance);
                        let new_direction = ray.direction.rotate(intersection.normal, PI);
                        let new_ray = objects::Ray::new(new_origin, new_direction);
                        phong * (1f64 - intersection.material.reflectivity) + self.raytrace_limited(new_ray, depth + 1) * intersection.material.reflectivity
                    } else {
                        phong
                    }
                },
                None => { self.background_color }
            }
        }
    }

    fn cast_ray(&self, ray: objects::Ray) -> Option<objects::Intersection> {
        let mut closest: Option<objects::Intersection> = Option::None;

        for o in &self.objects {
            match o.intersect(&ray) {
                Some(intersection) => {
                    // TODO: Didn't use matching because borrowing got weird. Fix.
                    if closest.is_some() {
                        if intersection.distance < closest.unwrap().distance {
                            closest = Some(intersection);
                        }
                    } else {
                        closest = Some(intersection);
                    }
                },
                None => {}
            }
        }

        return closest;
    }
}

