use objects::*;
use color::Color;

pub struct Scene<'a> {
    objects: Vec<&'a Intersectable>,
    lights: Vec<&'a Light>,
    background_color: Color,
}

impl<'a> Scene<'a> {
    pub fn new(
        objects: Vec<&'a Intersectable>,
        lights: Vec<&'a Light>,
        background_color: Color
    ) -> Scene<'a> {
        Scene { objects, lights, background_color }
    }

    pub fn raytrace(&self, ray: Ray) -> Color {
        match self.cast_ray(ray) {
            Some(intersection) => {
                self.phong(intersection)
            },
            None => { self.background_color }
        }
    }

    pub fn cast_ray(&self, ray: Ray) -> Option<Intersection> {
        let mut closest: Option<Intersection> = Option::None;

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

    fn phong(&self, intersection: Intersection) -> Color {
        let color = Color::new(0f64, 0f64, 0f64);
        for light in &self.lights {
            let unobstructedLight = self.cast_ray(Ray::new(intersection.location, light.position)).is_none();
            if (unobstructedLight) {

            }
        }
        intersection.material.ambient
    }

    /*
     for (vector<Light>::iterator light = scene->lights.begin(); light != scene->lights.end(); ++light)
    {
        vec3 lightdir;
        float distance;
        if (light->isDirectional)
        {
            lightdir = glm::normalize(light->coords);
            distance = FLT_MAX;
        }
        else
        {
            lightdir = glm::normalize(light->coords - intersection->point);
            distance = glm::distance(light->coords, intersection->point);
        }

        Intersection shadowIntersect = getIntersection(intersection->point, lightdir, distance, true);
        if (!shadowIntersect.isHit) //ray does not intersect with anything on the way to the light source
        {
	    vec3 diffuse = intersection->diffuse;
	    vec3 specular = intersection->specular;
	    float shininess = intersection->object->shininess;

	    float nDotL = glm::dot(intersection->normal, lightdir);
	    float nDotH = glm::dot(intersection->normal, glm::normalize(lightdir - ray));

            vec3 lambert = diffuse * light->color * max(nDotL, 0.0f);
	    vec3 phong = specular * light->color * pow(max(nDotH, 0.0f), shininess);

            vec3 atten = scene->attenuation;

            color += (lambert + phong) / (light->isDirectional ? 1.0f : (atten[0] + atten[1] * distance + atten[2] * distance * distance));
        }
    }
    */
}

