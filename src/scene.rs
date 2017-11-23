use math::*;
use core::*;
use color::Color;
use geometry::Geometry;
use util::Clamp;

pub struct Scene {
    objects: Vec<SceneObject>,
    lights: Vec<Light>,
    background_color: Color,
    pub depth_limit: u32,
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
        self.cast_ray(ray, 0)
    }

    fn get_closest_hit(&self, ray: &Ray, depth: u32) -> Option<SceneObjectHit> {
        if depth > self.depth_limit {
            None
        } else {
            let mut closest: Option<SceneObjectHit> = Option::None;

            for o in &self.objects {
                match o.intersect(&ray) {
                    Some(hit) => {
                        match hit.enter {
                            Some(_) => {
                                let intersection = hit.get_first_intersection();
                                closest = match closest {
                                    Some(closest_hit) => {
                                        if intersection.distance < closest_hit.hit.get_first_intersection().distance {
                                            Some(SceneObjectHit {
                                                hit,
                                                scene_object: &o,
                                            })
                                        } else {
                                            Some(closest_hit)
                                        }
                                    }
                                    None => Some(SceneObjectHit {
                                        hit,
                                        scene_object: &o,
                                    }),
                                };
                            }
                            None => {}
                        }
                    }
                    None => {}
                }
            }

            closest
        }
    }

    fn cast_ray(&self, ray: &Ray, depth: u32) -> Color {
        match self.get_closest_hit(ray, depth) {
            Some(ref object_hit) => self.get_color(ray, object_hit, depth),
            None => self.background_color,
        }
    }

    fn get_color(&self, ray: &Ray, object_hit: &SceneObjectHit, depth: u32) -> Color {
        let ref intersection = object_hit.hit.get_first_intersection();
        let material = object_hit.scene_object.texture.get_material(intersection);
        let mut reflection_fraction = material.reflectivity;
        let mut transmission_fraction = material.transmission.as_ref().map(|transmission| transmission.transmissivity).unwrap_or(0f64);
        let inside = object_hit.hit.enter.is_none();
        let normal = if inside { -intersection.normal } else { intersection.normal };
        let mut eta = 0f64;

        if material.transmission.is_some() && reflection_fraction > 0f64 {
            let transmission = material.transmission.unwrap();
            let (eta_i, eta_t) = if inside {
                (transmission.index_of_refraction, 1f64)
            } else {
                (1f64, transmission.index_of_refraction)
            };
            eta = eta_i / eta_t;
            let fresnel_reflection_fraction =
                self.get_fresnel_reflection_percentage(ray, &normal, eta_i, eta_t);
            reflection_fraction *= fresnel_reflection_fraction;
            transmission_fraction *= 1f64 - fresnel_reflection_fraction;
        }

        let phong_fraction = 1f64 - reflection_fraction - transmission_fraction;
        let mut color = BLACK;

        // TODO: increase other fractions if inside
        if !inside && phong_fraction > 0f64 {
            color += phong_fraction * self.get_visible_lights(intersection.nudged_location(normal))
                .iter()
                .fold(BLACK, |color, light| {
                    let light_direction = (light.position - intersection.location).as_normalized();
                    let normalized_normal = normal.as_normalized();
                    let diffuse_illumination = material.diffuse * light.color * normalized_normal.dot(&light_direction).max(0f64);
                    let specular_illumination = material.specular.0 * light.color
                        * normalized_normal.dot(&(light_direction - ray.direction).as_normalized()).max(0f64).powf(material.specular.1);
                    color + diffuse_illumination + specular_illumination
                });
        }

        if reflection_fraction > 0f64 {
            let new_direction = ray.direction.reflect(normal.as_vector());
            let ref new_ray = Ray::new(intersection.nudged_location(normal), new_direction);
            color += reflection_fraction * self.cast_ray(new_ray, depth + 1)
        }

        if transmission_fraction > 0f64 {
            let cos_i = -ray.direction.dot(&normal).clamp(-1f64, 1f64);
            let k = 1f64 - eta * eta * (1f64 - cos_i * cos_i);
            if k >= 0f64 {
                let direction = ray.direction * eta + (normal * (eta * cos_i - k.sqrt())).as_vector();
                let origin = intersection.nudged_location(-normal);
                let ref new_ray = Ray::new(origin, direction.as_normalized());
                color += transmission_fraction * self.cast_ray(new_ray, depth + 1)
            }
        }

        color
    }

    // https://www.scratchapixel.com/lessons/3d-basic-rendering/introduction-to-shading/reflection-refraction-fresnel
    // https://graphics.stanford.edu/courses/cs148-10-summer/docs/2006--degreve--reflection_refraction.pdf
    // TODO: Schlick?
    fn get_fresnel_reflection_percentage(&self, ray: &Ray, normal: &Normal, eta_i: f64, eta_t: f64) -> f64 {
        let cos_i = -ray.direction.dot(normal).clamp(-1f64, 1f64);
        let eta = eta_i / eta_t;
        let sin2_t = eta * eta * (1f64 - cos_i * cos_i);
        if sin2_t > 1f64 {
            1f64
        } else {
            let cos_t = (1f64 - sin2_t).sqrt();
            let r_orthagonal = (eta_i * cos_i - eta_t * cos_t) / (eta_i * cos_i + eta_t * cos_t);
            let r_parallel = (eta_t * cos_i - eta_i * cos_t) / (eta_t * cos_i + eta_i * cos_t);
            (r_orthagonal * r_orthagonal + r_parallel * r_parallel) / 2f64
        }
    }

    fn get_visible_lights(&self, point: Point) -> Vec<&Light> {
        self.lights
            .iter()
            .filter(|light| {
                let light_direction = (light.position - point).as_normalized();
                let ref ray = Ray::new(point, light_direction);
                self.get_closest_hit(ray, 0u32).is_none()
            })
            .collect::<Vec<&Light>>()
    }
}

