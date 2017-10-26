use objects;

pub struct Scene<'a> {
    objects: Vec<&'a objects::Intersectable>,
}

impl<'a> Scene<'a> {
    pub fn new(objects: Vec<&objects::Intersectable>) -> Scene {
        Scene { objects }
    }

    pub fn cast_ray(&'a self, ray: objects::Ray) -> Option<objects::Intersection> {
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

