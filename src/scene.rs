use objects::*;
use color::Color;

pub struct Scene<'a> {
    objects: Vec<&'a Intersectable>,
    lights: Vec<&'a Light>,
    background_color: Color,
    depth_limit: u32,
}

impl<'a> Scene<'a> {
    pub fn new(
        objects: Vec<&'a Intersectable>,
        lights: Vec<&'a Light>,
        background_color: Color,
        depth_limit: u32
    ) -> Scene<'a> {
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
                    let phong = self.phong(ray, intersection);
                    if intersection.material.reflectivity > 0f64 {
                        let new_origin = ray.at(intersection.distance);
                        let new_direction = ray.direction.reflect(intersection.normal);
                        let new_ray = Ray::new(new_origin, new_direction);
                        phong * (1f64 - intersection.material.reflectivity) + self.raytrace_depth_limited(new_ray, depth + 1) * intersection.material.reflectivity
                    } else {
                        phong
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
                    match &closest {
                        &Some(previous_intersection) => {
                            if intersection.distance < previous_intersection.distance {
                                closest = Some(intersection);
                            }
                        },
                        &None => {
                            closest = Some(intersection);
                        }
                    }
                },
                None => {}
            }
        }

        closest
    }

    fn phong(&self, ray: Ray, intersection: Intersection) -> Color {
        let material = intersection.material;
        let mut color = intersection.material.ambient;
        for light in &self.lights {
            let light_direction = (light.position - intersection.location).as_unit_vector();
            let unobstructed_light = self.cast_ray(Ray::new(intersection.location, light_direction)).is_none();
            if unobstructed_light {
                let diffuse_illumination = material.diffuse * light.color * intersection.normal.dot(light_direction).max(0f64);
                let specular_illumination = material.specular * light.color * intersection.normal.dot((light_direction - ray.direction).as_unit_vector()).max(0f64).powf(material.specular_exponent);
                color = color + diffuse_illumination + specular_illumination;
            }
        }
        color
    }
}

