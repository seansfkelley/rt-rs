use math::*;
use core::*;
use color::Color;
use util::Clamp;

pub struct Scene {
    objects: KdTree<SceneObject>,
    lights: Vec<Light>,
    background_color: Color,
    depth_limit: u32,
}

impl Scene {
    pub fn new(
        objects: KdTree<SceneObject>,
        lights: Vec<Light>,
        background_color: Color,
        depth_limit: u32
    ) -> Scene {
        Scene { objects, lights, background_color, depth_limit }
    }

    pub fn raytrace(&self, ray: Ray) -> Color {
        self.cast_ray(ray, 0)
    }

    fn cast_ray(&self, ray: Ray, depth: u32) -> Color {
        if depth > self.depth_limit {
            self.background_color
        } else {
            match self.objects.intersect(&ray) {
                Some(object_hit) => self.get_color(&ray, object_hit, depth),
                None => self.background_color,
            }
        }
    }

    fn get_color(&self, ray: &Ray, intersection: Intersection, depth: u32) -> Color {
        const NUDGE_FACTOR: f64 = 1e-10f64;

        let material = intersection.material.expect("scene intersections should always have a material");
        let mut reflection_fraction = material.reflectivity;
        let mut transmission_fraction = material.transmission.as_ref().map(|transmission| transmission.transmissivity).unwrap_or(0f64);

        let is_inside = intersection.normal.dot(&ray.direction) > 0f64;
        let normal = if is_inside { -intersection.normal.as_normalized() } else { intersection.normal.as_normalized() };
        let shading_normal = intersection.shading_normal.unwrap_or(normal);
        let location = intersection.location;

        let nudged_location = |normal: Normal| location + (normal * NUDGE_FACTOR).into_vector();

        let mut eta = 0f64;

        if material.transmission.is_some() && reflection_fraction > 0f64 {
            let transmission = material.transmission.as_ref().unwrap();
            let (eta_i, eta_t) = if is_inside {
                (transmission.index_of_refraction, 1f64)
            } else {
                (1f64, transmission.index_of_refraction)
            };
            eta = eta_i / eta_t;
            let fresnel_reflection_fraction =
                self.get_fresnel_reflection_percentage(ray, &shading_normal, eta_i, eta_t);
            reflection_fraction *= fresnel_reflection_fraction;
            transmission_fraction *= 1f64 - fresnel_reflection_fraction;
        }

        let phong_fraction = 1f64 - reflection_fraction - transmission_fraction;
        let mut color = BLACK;

        // TODO: increase other fractions if inside
        if !is_inside && phong_fraction > 0f64 {
            color += phong_fraction * self.get_visible_lights(nudged_location(normal))
                .iter()
                .fold(material.ambient, |color, light| {
                    let light_direction = (light.position - location).into_normalized();
                    let diffuse_illumination = material.diffuse * light.color * shading_normal.dot(&light_direction).max(0f64);
                    let specular_illumination = material.specular.0 * light.color
                        * shading_normal.dot(&(light_direction - ray.direction).into_normalized()).max(0f64).powf(material.specular.1);
                    color + diffuse_illumination + specular_illumination
                });
        }

        if reflection_fraction > 0f64 {
            let new_direction = ray.direction.reflect(shading_normal.as_vector());
            let new_ray = Ray::half_infinite(nudged_location(normal), new_direction);
            color += reflection_fraction * self.cast_ray(new_ray, depth + 1)
        }

        if transmission_fraction > 0f64 {
            let cos_i = -ray.direction.dot(&shading_normal).clamp(-1f64, 1f64);
            let k = 1f64 - eta * eta * (1f64 - cos_i * cos_i);
            if k >= 0f64 {
                let direction = ray.direction * eta + (shading_normal * (eta * cos_i - k.sqrt())).into_vector();
                let origin = nudged_location(-normal);
                let new_ray = Ray::half_infinite(origin, direction.as_normalized());
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
            let r_orthogonal = (eta_i * cos_i - eta_t * cos_t) / (eta_i * cos_i + eta_t * cos_t);
            let r_parallel = (eta_t * cos_i - eta_i * cos_t) / (eta_t * cos_i + eta_i * cos_t);
            (r_orthogonal * r_orthogonal + r_parallel * r_parallel) / 2f64
        }
    }

    fn get_visible_lights(&self, point: Point) -> Vec<&Light> {
        self.lights
            .iter()
            .filter(|light| {
                let light_direction = (light.position - point).into_normalized();
                let ray = Ray::half_infinite(point, light_direction);
                !self.objects.does_intersect(&ray)
            })
            .collect::<Vec<&Light>>()
    }
}

