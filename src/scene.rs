use objects::*;
use color::{ Color, BLACK };

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
                    let lighting = intersection.object.material().get_lighting(&ray, &intersection);

                    let mut color: Color = self.lights
                        .iter()
                        .filter(|light| {
                            let light_direction = (light.position - intersection.location).as_unit_vector();
                            self.cast_ray(Ray::new(intersection.location, light_direction)).is_none()
                        })
                        .fold(BLACK, |color, light| {
                            let light_direction = (light.position - intersection.location).as_unit_vector();
                            let diffuse_illumination = lighting.diffuse * light.color * intersection.normal.dot(light_direction).max(0f64);
                            let specular_illumination = lighting.specular.0 * light.color * intersection.normal.dot((light_direction - ray.direction).as_unit_vector()).max(0f64).powf(lighting.specular.1);
                            color + diffuse_illumination + specular_illumination
                        });

                    let reflectivity = lighting.reflective.1;

                    if reflectivity > 0f64 {
                        let new_origin = ray.at(intersection.distance);
                        let new_direction = ray.direction.reflect(intersection.normal);
                        let new_ray = Ray::new(new_origin, new_direction);
                        color = (1f64 - reflectivity) * color + reflectivity * self.raytrace_depth_limited(new_ray, depth + 1)
                    }

                    color
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

