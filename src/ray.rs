use crate::material::Material;
use crate::vec3::Vec3;
use crate::world::World;

pub enum Face {
    Front,
    Back,
}

pub struct HitRecord<'a> {
    pub p: Vec3,
    pub normal: Vec3,
    material: &'a Box<dyn Material>,
    t: f32,
    pub face: Face,
}

impl<'a> HitRecord<'a> {
    pub fn new(
        t: f32,
        p: Vec3,
        ray: &Ray,
        outward_normal: Vec3,
        material: &'a Box<dyn Material>,
    ) -> Self {
        let front_face = ray.direction.dot(&outward_normal) < 0.0;
        let (face, normal) = if front_face {
            (Face::Front, outward_normal)
        } else {
            (Face::Back, -outward_normal)
        };

        Self {
            p,
            normal,
            t,
            material,
            face,
        }
    }
}

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self { origin, direction }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + (t * self.direction)
    }

    pub fn color(&self, world: &World, depth: usize) -> Vec3 {
        if depth == 0 {
            return Vec3::new(0.0, 0.0, 0.0);
        }

        if let Some(hit_record) = self.hit_record_from_world(&world) {
            if let Some((attenuation, scattered)) = hit_record.material.scatter(self, &hit_record) {
                attenuation * scattered.color(&world, depth - 1)
            } else {
                Vec3::new(0.0, 0.0, 0.0)
            }
        } else {
            let unit_direction = self.direction.unit_vector();
            let t = 0.5 * (unit_direction.y + 1.0);
            (1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0)
        }
    }

    fn hit_record_from_world<'a>(&self, world: &'a World) -> Option<HitRecord<'a>> {
        let t_min = 0.001;
        let t_max = f32::INFINITY;

        let mut closest_so_far = t_max;
        let mut hit_record = None;

        for (surface, material) in &world.objects {
            if let Some(hr) = surface.hit(self, t_min, closest_so_far, &material) {
                closest_so_far = hr.t;
                hit_record = Some(hr);
            }
        }

        hit_record
    }
}
