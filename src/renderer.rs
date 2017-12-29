use std::collections::HashSet;
use rand::{ Rng, thread_rng };
use math::*;
use core::*;

pub struct Renderer {
    scene: Scene,
    parameters: RenderParamaters,
    camera: Camera,
}

lazy_static! {
    static ref BXDF_SAMPLE_TYPES: Vec<BxdfType> = vec![
        (TransportType::Reflective, SpectrumType::Diffuse),
        (TransportType::Reflective, SpectrumType::GlossySpecular),
        (TransportType::Transmissive, SpectrumType::Diffuse),
        (TransportType::Transmissive, SpectrumType::GlossySpecular),
    ];
}

impl Renderer {
    pub fn new(scene: Scene, parameters: RenderParamaters, camera: Camera) -> Renderer {
        Renderer {
            scene,
            parameters,
            camera,
        }
    }

    pub fn with_camera(self, camera: Camera) -> Renderer {
        Renderer {
            scene: self.scene,
            parameters: self.parameters,
            camera,
        }
    }

    pub fn render_pixel(&self, image_x: u32, image_y: u32) -> Color {
        let antialias = self.parameters.antialias;
        if antialias == 1u32 {
            self.Li(self.generate_ray(image_x, image_y), 0)
        } else {
            let mut rng = thread_rng();

            let test_points = {
                let max = antialias - 1;
                vec![
                    (0u32, 0u32),
                    (0u32, max),
                    (max, 0u32),
                    (max, max),
                ]
            };

            let test_colors = test_points
                .iter()
                .map(|&(sample_x, sample_y)| {
                    self.Li(self.generate_supersampling_ray(image_x, image_y, sample_x, sample_y, &mut rng), 0)
                })
                .collect::<Vec<Color>>();

            let mut color: Color = test_colors.iter().fold(Color::BLACK.clone(), |result, &color| result + color);

            if min_vs_max(&test_colors) > self.parameters.antialias_tolerance {
                let test_point_set: HashSet<&(u32, u32)> = test_points.iter().collect();
                for sample_x in 0..antialias {
                    for sample_y in 0..antialias {
                        if !test_point_set.contains(&(sample_x, sample_y)) {
                            color += self.Li(self.generate_supersampling_ray(image_x, image_y, sample_x, sample_y, &mut rng), 0);
                        }
                    }
                }
                color / (antialias * antialias) as f64
            } else {
                color / 4f64
            }
        }
    }

    fn generate_ray(&self, image_x: u32, image_y: u32) -> Ray {
        self.camera.get_ray(image_x as f64, image_y as f64)
    }

    fn generate_supersampling_ray(&self, image_x: u32, image_y: u32, sample_x: u32, sample_y: u32, rng: &mut Rng) -> Ray {
        let antialias = self.parameters.antialias;

        let (x_min, x_max, y_min, y_max) = (
            sample_x as f64 / antialias as f64,
            (1f64 + sample_x as f64) / antialias as f64,
            sample_y as f64 / antialias as f64,
            (1f64 + sample_y as f64) / antialias as f64
        );

        let x_jitter = rng.next_f64() * (x_max - x_min) + x_min;
        let y_jitter = rng.next_f64() * (y_max - y_min) + y_min;

        self.camera.get_ray(image_x as f64 + x_jitter, image_y as f64 + y_jitter)
    }

    #[allow(non_snake_case)] // Name from pbrt.
    fn Li(&self, ray: Ray, depth: u32) -> Color {
        if depth > self.parameters.depth_limit {
            self.parameters.background_color
        } else {
            match self.scene.objects.intersect(&ray) {
                Some(object_hit) => self.integrate_direct_lighting(ray, object_hit, depth),
                None => self.parameters.background_color,
            }
        }
    }

    fn integrate_direct_lighting(&self, ray: Ray, intersection: Intersection, depth: u32) -> Color {
        const NUDGE_FACTOR: f64 = 1e-10f64;

        #[allow(non_snake_case)]
        let mut L = Color::BLACK.clone();

        let bsdf = intersection.material.as_ref().expect("scene intersections should always have a material").get_bsdf(&intersection);

        // pbrt doesn't seem to normalize direction -- is it implicitly normalized already?
        let w_o = -ray.direction;
        let p = intersection.location;
        // pbrt uses the explictly already-normalized normal, so we just do that work here instead.
        let n = {
            match intersection.shading_normal {
                Some(normal) => normal,
                None => intersection.normal,
            }
        }.as_normalized();

        // TODO: Emission from area lights.
        // TODO: When do we figure shit out about being inside the shape...?

        for light in &self.scene.lights {
            L += self.estimate_light(light, &bsdf, p, n, w_o) + self.estimate_bsdf(light, &bsdf, p, n, w_o);
        }

        // let mut reflection_fraction = material.reflectivity;
        // let mut transmission_fraction = material.transmission.as_ref().map(|transmission| transmission.transmissivity).unwrap_or(0f64);

        // let is_inside = intersection.normal.dot(&ray.direction) > 0f64;
        // let normal = (if is_inside { -intersection.normal } else { intersection.normal }).into_normalized();
        // let shading_normal = intersection.shading_normal
        //     .map(|n| (if is_inside { -n } else { n }).into_normalized()).unwrap_or(normal);
        // let location = intersection.location;

        // let nudged_location = |normal: Normal| location + (normal * NUDGE_FACTOR).into_vector();

        // let mut eta = 0f64;

        // if material.transmission.is_some() && reflection_fraction > 0f64 {
        //     let transmission = material.transmission.as_ref().unwrap();
        //     let (eta_i, eta_t) = if is_inside {
        //         (transmission.index_of_refraction, 1f64)
        //     } else {
        //         (1f64, transmission.index_of_refraction)
        //     };
        //     eta = eta_i / eta_t;
        //     let fresnel_reflection_fraction =
        //         self.get_fresnel_reflection_percentage(&ray, &shading_normal, eta_i, eta_t);
        //     reflection_fraction *= fresnel_reflection_fraction;
        //     transmission_fraction *= 1f64 - fresnel_reflection_fraction;
        // }

        // let phong_fraction = 1f64 - reflection_fraction - transmission_fraction;
        // let mut color = Color::BLACK;

        // // TODO: increase other fractions if inside
        // if !is_inside && phong_fraction > 0f64 {
        //     color += phong_fraction * self.get_visible_lights(nudged_location(normal))
        //         .iter()
        //         .fold(material.ambient, |color, light| {
        //             let light_direction = (light.position - location).into_normalized();
        //             let diffuse_illumination = material.diffuse * light.color * shading_normal.dot(&light_direction).max(0f64);
        //             let specular_illumination = material.specular.0 * light.color
        //                 * shading_normal.dot(&(light_direction - ray.direction).into_normalized()).max(0f64).powf(material.specular.1);
        //             color + diffuse_illumination + specular_illumination
        //         });
        // }

        // if reflection_fraction > 0f64 {
        //     let new_direction = ray.direction.reflect(shading_normal.as_vector());
        //     let new_ray = Ray::half_infinite(nudged_location(normal), new_direction);
        //     color += reflection_fraction * self.Li(new_ray, depth + 1)
        // }

        // if transmission_fraction > 0f64 {
        //     let cos_i = -ray.direction.dot(&shading_normal).clamp(-1f64, 1f64);
        //     let k = 1f64 - eta * eta * (1f64 - cos_i * cos_i);
        //     if k >= 0f64 {
        //         let direction = ray.direction * eta + (shading_normal * (eta * cos_i - k.sqrt())).into_vector();
        //         let origin = nudged_location(-normal);
        //         let new_ray = Ray::half_infinite(origin, direction.as_normalized());
        //         color += transmission_fraction * self.Li(new_ray, depth + 1)
        //     }
        // }

        L
    }

    fn estimate_light(&self, light: &LightType, bsdf: &Bsdf, p: Point, n: Normal, w_o: Vec3) -> Color {
        let LightSample { l: l_i, w_i, pdf: light_pdf, visibility_ray } = light.choose_and_sample_radiance(p);
        if light_pdf > 0f64 && l_i.is_nonzero() {
            let bsdf_transport = bsdf.evaluate(w_o, w_i, &BXDF_SAMPLE_TYPES);

            if bsdf_transport.is_nonzero() && !self.scene.objects.does_intersect(&visibility_ray) {
                // TODO: Transmittance.
                match light {
                    // If the light is a delta light, we know that w_i is spot on (because that's how delta lights work)
                    // and thus multiple importance sampling isn't going to improve our results. Don't weight it.
                    &LightType::Delta(_) => {
                        bsdf_transport * l_i * (w_i.dot(&n).abs() / light_pdf)
                    }
                    // If the light is not a delta light, we will try sampling again later. For now, yield the contribution
                    // of this light sample weighted by its likelihood.
                    &LightType::Area(_) => {
                        let bsdf_pdf = bsdf.pdf(w_o, w_i, &BXDF_SAMPLE_TYPES);
                        let weight = variance_power_heuristic(light_pdf, 1, bsdf_pdf, 1);
                        bsdf_transport * l_i * (w_i.dot(&n) * weight / light_pdf)
                    }
                }
            } else {
                Color::BLACK
            }
        } else {
            Color::BLACK
        }
    }

    fn estimate_bsdf(&self, light: &LightType, bsdf: &Bsdf, p: Point, n: Normal, w_o: Vec3) -> Color {
        match light {
            // If the light is a delta light, bsdf sampling will never hit it. Abort.
            &LightType::Delta(_) => {
                Color::BLACK
            }
            &LightType::Area(ref light) => {
                let mut rng = thread_rng();

                match bsdf.choose_and_evaluate(w_o, &mut rng, &BXDF_SAMPLE_TYPES) {
                    Some((BxdfSample { color: bsdf_transport, pdf: bsdf_pdf, w_i, }, spectrum_type)) => {
                        if bsdf_pdf > 0f64 && bsdf_transport.is_nonzero() {
                            let weight = match spectrum_type {
                                // TODO: I thought perfect specular was hard to aim, why does this automatically get max weight?
                                SpectrumType::PerfectSpecular => { 1f64 }
                                _ => {
                                    let light_pdf = light.pdf(p, w_i);
                                    if light_pdf == 0f64 {
                                        0f64
                                    } else {
                                        variance_power_heuristic(bsdf_pdf, 1, light_pdf, 1)
                                    }
                                }
                            };
                            let l_i = match self.scene.objects.intersect(&Ray::half_infinite(p, w_i)) {
                                Some(intersection) => {
                                    // TODO: We'll want to modify Intersection to allow us to check if we hit the right thing.
                                    Color::WHITE
                                }
                                // TODO: This branch should be used for infinite area lights iff we implement them.
                                None => { Color::BLACK }
                            };
                            if l_i.is_nonzero() {
                                // TODO: Transmittance.
                                bsdf_transport * l_i * (w_i.dot(&n) * weight / bsdf_pdf)
                            } else {
                                Color::BLACK
                            }
                        } else {
                            Color::BLACK
                        }
                    }
                    None => { Color::BLACK }
                }
            }
        }
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
}
