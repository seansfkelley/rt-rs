use objects;
use color::Color;

pub struct Scene<'a> {
    objects: Vec<&'a objects::Intersectable>,
    lights: Vec<&'a objects::Light>,
    background_color: Color,
}

impl<'a> Scene<'a> {
    pub fn new(
        objects: Vec<&'a objects::Intersectable>,
        lights: Vec<&'a objects::Light>,
        background_color: Color
    ) -> Scene<'a> {
        Scene { objects, lights, background_color }
    }

    pub fn raytrace(&self, ray: objects::Ray) -> Color {
        match self.cast_ray(ray) {
            Some(intersection) => { intersection.material.ambient },
            None => { self.background_color }
        }
    }

    pub fn cast_ray(&self, ray: objects::Ray) -> Option<objects::Intersection> {
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

