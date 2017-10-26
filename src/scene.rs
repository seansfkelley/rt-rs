use objects;

pub struct Scene<'a> {
    objects: Vec<&'a objects::Intersectable>,
    lights: Vec<&'a objects::Light>,
}

impl<'a> Scene<'a> {
    pub fn new(objects: Vec<&'a objects::Intersectable>, lights: Vec<&'a objects::Light>) -> Scene<'a> {
        Scene { objects, lights }
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

