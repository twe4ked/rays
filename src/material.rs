use crate::ray::{HitRecord, Normal, Ray};
use crate::vec3::Vec3;

pub trait Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)>;
}

pub struct Lambertian {
    pub albedo: Vec3,
}

impl Lambertian {
    pub fn new(albedo: Vec3) -> Self {
        Self { albedo }
    }
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

pub struct Dielectric {
    ref_idx: f32,
}

impl Dielectric {
    pub fn new(ref_idx: f32) -> Self {
        Self { ref_idx }
    }
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<(Vec3, Ray)> {
        fn refract(uv: &Vec3, n: &Vec3, etai_over_etat: f32) -> Vec3 {
            let cos_theta = -uv.dot(n);
            let r_out_parallel = etai_over_etat * (*uv + cos_theta * *n);
            let r_out_perp = -(1.0 - r_out_parallel.length_squared()).sqrt() * *n;
            r_out_parallel + r_out_perp
        }

        let (normal, front_face) = match hit_record.normal {
            Normal::FrontFace(normal) => (normal, true),
            Normal::BackFace(normal) => (normal, false),
        };

        let attenuation = Vec3::new(1.0, 1.0, 1.0);
        let etai_over_etat = if front_face {
            1.0 / self.ref_idx
        } else {
            self.ref_idx
        };

        let unit_direction = ray_in.direction.unit_vector();

        let cos_theta = f32::min(-unit_direction.dot(&normal), 1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let rand_f32 = || crate::RAND.with(|r| r.borrow_mut().next_f32());
        let reflect_prob = schlick(cos_theta, etai_over_etat);

        if etai_over_etat * sin_theta > 1.0 || rand_f32() < reflect_prob {
            let reflected = reflect(&unit_direction, &normal);
            let scattered = Ray::new(hit_record.p, reflected);
            Some((attenuation, scattered))
        } else {
            let refracted = refract(&unit_direction, &normal, etai_over_etat);
            let scattered = Ray::new(hit_record.p, refracted);
            Some((attenuation, scattered))
        }
    }
}

// Random Vec3 within a sphere
fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3::random(-1.0, 1.0);
        if p.length_squared() < 1.0 {
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

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2.0 * v.dot(&n) * (*n)
}

// Schlick approximation
fn schlick(cosine: f32, ref_idx: f32) -> f32 {
    let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
    r0 = r0 * r0;
    r0 + (1.0 - r0) * (1.0 - cosine).powf(5.0)
}
