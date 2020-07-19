use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::rand::{rand, rand_between};
use crate::surface::Sphere;
use crate::surface::Surface;
use crate::vec3::Vec3;

pub struct World {
    pub objects: Vec<(Box<dyn Surface + Sync>, Box<dyn Material + Sync>)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, surface: Box<dyn Surface + Sync>, material: Box<dyn Material + Sync>) {
        self.objects.push((surface, material));
    }

    pub fn random_scene() -> Self {
        let mut world = World::new();

        let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
        world.add(
            Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0)),
            Box::new(ground_material),
        );

        for a in -11..11 {
            for b in -11..11 {
                let choose_material = rand();
                let center = Vec3::new(a as f32 + 0.9 * rand(), 0.2, b as f32 + 0.9 * rand());

                if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                    if choose_material < 0.8 {
                        // Diffuse
                        let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                        world.add(
                            Box::new(Sphere::new(center, 0.2)),
                            Box::new(Lambertian::new(albedo)),
                        );
                    } else if choose_material < 0.95 {
                        // Metal
                        let albedo = Vec3::random(0.5, 1.0);
                        let fuzz = rand_between(0.0, 0.5);
                        world.add(
                            Box::new(Sphere::new(center, 0.2)),
                            Box::new(Metal::new(albedo, fuzz)),
                        );
                    } else {
                        // Glass
                        world.add(
                            Box::new(Sphere::new(center, 0.2)),
                            Box::new(Dielectric::new(1.5)),
                        );
                    }
                }
            }
        }

        world.add(
            Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0)),
            Box::new(Dielectric::new(1.5)),
        );

        world.add(
            Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0)),
            Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
        );

        world.add(
            Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0)),
            Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
        );

        world
    }
}
