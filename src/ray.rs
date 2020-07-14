use crate::vec3::Vec3;

enum Normal {
    FrontFace(Vec3),
    BackFace(Vec3),
}

pub struct HitRecord {
    p: Vec3,
    normal: Normal,
    t: f32,
}

impl HitRecord {
    fn new(t: f32, p: Vec3, ray: &Ray, outward_normal: Vec3) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let normal = if front_face {
            Normal::FrontFace(outward_normal)
        } else {
            Normal::BackFace(-outward_normal)
        };

        Self { p, normal, t }
    }

    fn from_world(ray: &Ray, world: &[Box<dyn Surface>]) -> Option<Self> {
        let t_min = 0.0;
        let t_max = f32::INFINITY;

        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for surface in world {
            if let Some(hr) = surface.hit(ray, t_min, closest_so_far) {
                closest_so_far = hr.t;
                hit_record = Some(hr);
            }
        }

        hit_record
    }
}

pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
}

pub trait Surface {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord>;
}

impl Surface for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - self.radius * self.radius;

        let hit_record_for_root = |root: f32| -> Option<HitRecord> {
            let temp = (-half_b + root) / a;

            if temp < t_max && temp > t_min {
                let t = temp;
                let p = ray.at(t);
                let outward_normal = (p - self.center) / self.radius;

                Some(HitRecord::new(t, p, ray, outward_normal))
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

pub struct Ray {
    origin: Vec3,
    direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + (t * self.direction)
    }

    pub fn color(&self, world: &[Box<dyn Surface>]) -> Vec3 {
        if let Some(hr) = HitRecord::from_world(self, &world) {
            match hr.normal {
                Normal::FrontFace(normal) => 0.5 * (normal + Vec3::new(1.0, 1.0, 1.0)),
                Normal::BackFace(_) => todo!(),
            }
        } else {
            let unit_direction = self.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }
}
