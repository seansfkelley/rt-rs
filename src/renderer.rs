use std::collections::HashSet;
use rand::{ Rng, ThreadRng, thread_rng };
use scene::Scene;
use core::*;

pub struct Renderer {
    scene: Scene,
    parameters: RenderParamaters,
    camera: Camera,
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
            self.scene.raytrace(self.camera.get_ray(image_x as f64, image_y as f64))
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
                .map(|&(sample_x, sample_y)| self.sample_pixel_once(image_x, image_y, sample_x, sample_y, &mut rng))
                .collect::<Vec<Color>>();

            let mut color: Color = test_colors.iter().fold(color::BLACK.clone(), |result, &color| result + color);

            if min_vs_max(&test_colors) > self.parameters.antialias_tolerance {
                let test_point_set: HashSet<&(u32, u32)> = test_points.iter().collect();
                for sample_x in 0..antialias {
                    for sample_y in 0..antialias {
                        if !test_point_set.contains(&(sample_x, sample_y)) {
                            color += self.sample_pixel_once(image_x, image_y, sample_x, sample_y, &mut rng);
                        }
                    }
                }
                color / (antialias * antialias) as f64
            } else {
                color / 4f64
            }
        }
    }

    fn sample_pixel_once(&self, image_x: u32, image_y: u32, sample_x: u32, sample_y: u32, rng: &mut ThreadRng) -> Color {
        let antialias = self.parameters.antialias;

        let (x_min, x_max, y_min, y_max) = (
            sample_x as f64 / antialias as f64,
            (1f64 + sample_x as f64) / antialias as f64,
            sample_y as f64 / antialias as f64,
            (1f64 + sample_y as f64) / antialias as f64
        );

        let x_jitter = rng.next_f64() * (x_max - x_min) + x_min;
        let y_jitter = rng.next_f64() * (y_max - y_min) + y_min;

        self.scene.raytrace(self.camera.get_ray(image_x as f64 + x_jitter, image_y as f64 + y_jitter))
    }
}
