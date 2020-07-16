use crate::ray::{HitRecord, Normal, Ray};
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Material for Lambertian {
    fn scatter(&self, _ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        let normal = match hit_record.normal {
            Normal::FrontFace(normal) | Normal::BackFace(normal) => normal,
        };
        let scatter_direction = normal + random_unit_vec3();
        let scattered = Ray::new(hit_record.p, scatter_direction);
        let attenuation = self.albedo;

        Some((attenuation, scattered))
    }
}

pub struct Metal {
    pub albedo: Vec3,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f32) -> Self {
        assert!(fuzz <= 1.0);

        Self { albedo, fuzz }
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
            *v - 2.0 * v.dot(&n) * (*n)
        }

        match hit_record.normal {
            Normal::FrontFace(normal) | Normal::BackFace(normal) => {
                let reflected = reflect(&ray_in.direction.unit_vector(), &normal);
                let scattered = Ray::new(
                    hit_record.p,
                    reflected + self.fuzz * random_in_unit_sphere(),
                );
                let attenuation = self.albedo;

                if scattered.direction.dot(&normal) > 0.0 {
                    Some((attenuation, scattered))
                } else {
                    None
                }
            }
        }
    }
}

// Random Vec3 within a sphere
fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if !(p.length_squared() >= 1.0) {
            return p;
        }
    }
}

// Lambertian distribution
fn random_unit_vec3() -> Vec3 {
    let rand = |min, max| crate::RAND.with(|r| r.borrow_mut().next_between_f32(min, max));

    let a = rand(0.0, 2.0 * std::f32::consts::PI);
    let z = rand(-1.0, 1.0);
    let r = (1.0 - z * z).sqrt();

    Vec3 {
        x: r * a.cos(),
        y: r * a.sin(),
        z,
    }
}

// Hemispherical scattering
#[allow(dead_code)]
fn random_in_hemisphere(normal: &Vec3) -> Vec3 {
    let in_unit_sphere = random_in_unit_sphere();

    // In the same hemisphere as the normal
    if in_unit_sphere.dot(normal) > 0.0 {
        in_unit_sphere
    } else {
        -in_unit_sphere
    }
}
