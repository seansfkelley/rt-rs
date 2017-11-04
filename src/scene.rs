use objects::*;
use color::{Color, BLACK};

pub struct Scene {
    objects: Vec<Box<SceneObject>>,
    lights: Vec<Box<Light>>,
    background_color: Color,
    depth_limit: u32,
}

static DEBUG_SHADOW_COLOR: Color = Color { r: 0f64, g: 0f64, b: 1f64 };
static DEBUG_BASE_COLOR: Color = Color { r: 1f64, g: 0f64, b: 0f64 };
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

    fn raytrace_depth_limited(&self, ray: Ray, depth: u32) -> Color {
        if depth > self.depth_limit {
            self.background_color
        } else {
            match self.cast_ray(ray) {
                Some(hit) => {
                    let intersection = hit.enter.as_ref().unwrap();
                    let lighting = hit.object.material().get_lighting(&intersection);

                    let mut color: Color = self.lights
                        .iter()
                        .filter(|light| {
                            let adjusted_location = intersection.location - (ray.direction * 1e-9f64);
                            let light_direction = (light.position - adjusted_location).as_unit_vector();
                            self.cast_ray(Ray::new(adjusted_location, light_direction)).map(|hit| hit.enter).is_none()
                        })
                        .fold(BLACK, |color, light| {
                            if hit.debug {
                                color + (DEBUG_SHADOW_COLOR / (self.lights.len() as f64))
                            } else {
                                let light_direction = (light.position - intersection.location).as_unit_vector();
                                let diffuse_illumination = lighting.diffuse * light.color * intersection.normal.dot(light_direction).max(0f64);
                                let specular_illumination = lighting.specular.0 * light.color * intersection.normal.dot((light_direction - ray.direction).as_unit_vector()).max(0f64).powf(lighting.specular.1);
                                color + diffuse_illumination + specular_illumination
                            }
                        });

                    if hit.debug {
                        color + DEBUG_BASE_COLOR
                    } else {
                        let reflectivity = lighting.reflective.1;

                        if reflectivity > 0f64 {
                            let new_origin = ray.at(intersection.distance);
                            let new_direction = ray.direction.reflect(intersection.normal);
                            let new_ray = Ray::new(new_origin, new_direction);
                            color = (1f64 - reflectivity) * color + reflectivity * self.raytrace_depth_limited(new_ray, depth + 1)
                        }

                        color
                    }
                }
                None => { self.background_color }
            }
        }
    }

    fn cast_ray(&self, ray: Ray) -> Option<Hit> {
        let mut closest: Option<Hit> = Option::None;

        for o in &self.objects {
            match o.intersect(&ray) {
                Some(hit) => {
                    if hit.enter.is_some() {
                        if closest.is_some() {
                            if hit.enter.as_ref().unwrap().distance < closest.as_ref().unwrap().enter.as_ref().unwrap().distance {
                                closest = Some(hit);
                            }
                        } else {
                            closest = Some(hit);
                        }
                    }
                },
                None => {},
            }
        }

        closest
    }
}

