use objects::*;
use color::Color;

pub struct Scene {
    objects: Vec<Box<SceneObject>>,
    lights: Vec<Box<Light>>,
    background_color: Color,
    depth_limit: u32,
}

impl Scene {
    pub fn new(
        objects: Vec<Box<SceneObject>>,
        lights: Vec<Box<Light>>,
        background_color: Color,
        depth_limit: u32
    ) -> Scene {
        Scene { objects, lights, background_color, depth_limit }
    }

    pub fn raytrace(&self, ray: Ray) -> Color {
        self.raytrace_depth_limited(ray, 0)
    }

    pub fn raytrace_depth_limited(&self, ray: Ray, depth: u32) -> Color {
        if depth > self.depth_limit {
            self.background_color
        } else {
            match self.cast_ray(ray) {
                Some(intersection) => {
                    let visible_lights: Vec<&Light> = self.lights
                        .iter()
                        .filter(|light| {
                            let light_direction = (light.position - intersection.location).as_unit_vector();
                            self.cast_ray(Ray::new(intersection.location, light_direction)).is_none()
                        })
                        .map(|light| light.as_ref())
                        .collect();

                    let material = intersection.object.material();
                    let color = material.get_color(&ray, &intersection, visible_lights);
                    if material.reflectivity() > 0f64 {
                        let new_origin = ray.at(intersection.distance);
                        let new_direction = ray.direction.reflect(intersection.normal);
                        let new_ray = Ray::new(new_origin, new_direction);
                        color * (1f64 - material.reflectivity()) + self.raytrace_depth_limited(new_ray, depth + 1) * material.reflectivity()
                    } else {
                        color
                    }
                },
                None => { self.background_color }
            }
        }
    }

    fn cast_ray(&self, ray: Ray) -> Option<Intersection> {
        let mut closest: Option<Intersection> = Option::None;

        for o in &self.objects {
            match o.intersect(&ray) {
                Some(intersection) => {
                    if closest.is_some() {
                        if intersection.distance < closest.as_ref().unwrap().distance {
                            closest = Some(intersection);
                        }
                    } else {
                        closest = Some(intersection);
                    }
                },
                None => {}
            }
        }

        closest
    }
}

