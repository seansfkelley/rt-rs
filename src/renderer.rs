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
    static ref BXDF_REFLECTION_TYPES: Vec<BxdfType> = vec![
        (TransportType::Reflective, SpectrumType::PerfectSpecular),
    ];

    static ref BXDF_TRANSMISSION_TYPES: Vec<BxdfType> = vec![
        (TransportType::Transmissive, SpectrumType::PerfectSpecular),
    ];

    static ref BXDF_SURFACE_TYPES: Vec<BxdfType> = vec![
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
        let w_o = -ray.direction.as_normalized();
        // pbrt uses the explictly already-normalized normal, so we just do that work here instead.
        let n = {
            match intersection.shading_normal {
                Some(normal) => normal,
                None => intersection.normal,
            }
        }.as_normalized();
        let p = intersection.location + intersection.normal * NUDGE_FACTOR;

        // TODO: Emission from area lights.
        // TODO: When do we figure shit out about being inside the shape...?

        for light in &self.scene.lights {
            L += self.estimate_light(light, &bsdf, p, n, w_o) + self.estimate_bsdf(light, &bsdf, p, n, w_o);
        }

        L += self.integrate_perfect_specular_transport(&bsdf, p, n, w_o, &BXDF_REFLECTION_TYPES, depth);
        L += self.integrate_perfect_specular_transport(&bsdf, p, n, w_o, &BXDF_TRANSMISSION_TYPES, depth);

        L
    }

    fn estimate_light(&self, light: &LightType, bsdf: &Bsdf, p: Point, n: Normal, w_o: Vec3) -> Color {
        let LightSample { l: l_i, w_i, pdf: light_pdf, visibility_ray } = light.choose_and_sample_radiance(p);
        if light_pdf > 0f64 && l_i.is_nonzero() {
            let bsdf_transport = bsdf.evaluate(w_o, w_i, &BXDF_SURFACE_TYPES);

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
                        let bsdf_pdf = bsdf.pdf(w_o, w_i, &BXDF_SURFACE_TYPES);
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

                match bsdf.choose_and_evaluate(w_o, &mut rng, &BXDF_SURFACE_TYPES) {
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
                            if weight > 0f64 {
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
                        } else {
                            Color::BLACK
                        }
                    }
                    None => { Color::BLACK }
                }
            }
        }
    }

    fn integrate_perfect_specular_transport(&self, bsdf: &Bsdf, p: Point, n: Normal, w_o: Vec3, bxdf_types: &Vec<BxdfType>, depth: u32) -> Color {
        if depth == self.parameters.depth_limit {
            Color::BLACK
        } else {
            let mut rng = thread_rng();
            match bsdf.choose_and_evaluate(w_o, &mut rng, bxdf_types) {
                Some((BxdfSample { color: bsdf_transport, pdf, w_i, }, _)) => {
                    if pdf > 0f64 && bsdf_transport.is_nonzero() && w_i.dot(&n) != 0f64 {
                        bsdf_transport * self.Li(Ray::half_infinite(p, w_i), depth + 1) * (w_i.dot(&n).abs() / pdf)
                    } else {
                        Color::BLACK
                    }
                }
                None => { Color::BLACK }
            }
        }
    }
}
