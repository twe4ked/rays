use crate::material::Material;
use crate::ray::{HitRecord, Ray};
use crate::vec3::Vec3;

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32) -> Self {
        Self { center, radius }
    }
}

pub trait Surface {
    fn hit<'a>(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        material: &'a Box<dyn Material>,
    ) -> Option<HitRecord<'a>>;
}

impl Surface for Sphere {
    fn hit<'a>(
        &self,
        ray: &Ray,
        t_min: f32,
        t_max: f32,
        material: &'a Box<dyn Material>,
    ) -> Option<HitRecord<'a>> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let hit_record_for_root = |root| {
            let temp = (-half_b + root) / a;

            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.at(t);
                let outward_normal = (p - self.center) / self.radius;

                Some(HitRecord::new(t, p, ray, outward_normal, material))
            } else {
                None
            }
        };

        let discriminant = half_b * half_b - a * c;

        if discriminant > 0.0 {
            let root = discriminant.sqrt();
            [-root, root].iter().find_map(|&r| hit_record_for_root(r))
        } else {
            None
        }
    }
}
