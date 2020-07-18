// https://raytracing.github.io/books/RayTracingInOneWeekend.html

mod camera;
mod material;
mod ppm;
mod rand;
mod ray;
mod surface;
mod vec3;

use camera::Camera;
use material::{Dielectric, Lambertian, Material, Metal};
use rand::Rand;
use surface::{Sphere, Surface};
use vec3::Vec3;

use std::cell::RefCell;
use std::io;

thread_local! { static RAND: RefCell<Rand> = RefCell::new(Rand::new_from_time()); }

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    let camera = {
        let look_from = Vec3::new(13.0, 2.0, 3.0);
        let look_at = Vec3::new(0.0, 0.0, 0.0);
        let v_up = Vec3::new(0.0, 1.0, 0.0);

        let focus_dist = 10.0;
        let aperture = 0.1;

        Camera::new(
            look_from,
            look_at,
            v_up,
            20.0,
            aspect_ratio,
            aperture,
            focus_dist,
        )
    };

    let mut stdout = io::stdout();

    ppm::write_header(&mut stdout, image_width, image_height)?;

    let world = random_scene();

    let samples_per_pixel = 100;
    let max_depth = 50;

    let rand = || crate::RAND.with(|r| r.borrow_mut().next_f32());

    for j in (0..image_height as usize).rev() {
        eprint!(".");

        for i in 0..image_width as usize {
            let mut color = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rand()) / (image_width as f32 - 1.0);
                let v = (j as f32 + rand()) / (image_height as f32 - 1.0);
                let ray = camera.get_ray(u, v);
                color = color + ray.color(&world, max_depth);
            }

            color = translate_color(&color, samples_per_pixel as _);

            ppm::write_color(&mut stdout, &color)?;
        }
    }

    eprintln!("\n");
    eprintln!("Finished");

    Ok(())
}

// (
//     Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
//     Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
// ),

fn random_scene() -> Vec<(Box<dyn Surface>, Box<dyn Material>)> {
    let mut world: Vec<(Box<dyn Surface>, Box<dyn Material>)> = Vec::new();

    let ground_material = Lambertian::new(Vec3::new(0.5, 0.5, 0.5));
    world.push((
        Box::new(Sphere::new(Vec3::new(0.0, -1000.0, 0.0), 1000.0)),
        Box::new(ground_material),
    ));

    let rand = || crate::RAND.with(|r| r.borrow_mut().next_f32());
    let rand_between = |min, max| crate::RAND.with(|r| r.borrow_mut().next_between_f32(min, max));

    for a in -11..11 {
        for b in -11..11 {
            let choose_material = rand();
            let center = Vec3::new(a as f32 + 0.9 * rand(), 0.2, b as f32 + 0.9 * rand());

            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_material < 0.8 {
                    // Diffuse
                    let albedo = Vec3::random(0.0, 1.0) * Vec3::random(0.0, 1.0);
                    world.push((
                        Box::new(Sphere::new(center, 0.2)),
                        Box::new(Lambertian::new(albedo)),
                    ));
                } else if choose_material < 0.95 {
                    // Metal
                    let albedo = Vec3::random(0.5, 1.0);
                    let fuzz = rand_between(0.0, 0.5);
                    world.push((
                        Box::new(Sphere::new(center, 0.2)),
                        Box::new(Metal::new(albedo, fuzz)),
                    ));
                } else {
                    // Glass
                    world.push((
                        Box::new(Sphere::new(center, 0.2)),
                        Box::new(Dielectric::new(1.5)),
                    ));
                }
            }
        }
    }

    world.push((
        Box::new(Sphere::new(Vec3::new(0.0, 1.0, 0.0), 1.0)),
        Box::new(Dielectric::new(1.5)),
    ));

    world.push((
        Box::new(Sphere::new(Vec3::new(-4.0, 1.0, 0.0), 1.0)),
        Box::new(Lambertian::new(Vec3::new(0.4, 0.2, 0.1))),
    ));

    world.push((
        Box::new(Sphere::new(Vec3::new(4.0, 1.0, 0.0), 1.0)),
        Box::new(Metal::new(Vec3::new(0.7, 0.6, 0.5), 0.0)),
    ));

    world
}

fn translate_color(color: &Vec3, samples_per_pixel: f32) -> Vec3 {
    let scale = 1.0 / samples_per_pixel;

    // Divide the color total by the number of samples
    let color = *color * scale;

    // Gamma-correct for gamma=2.0
    Vec3 {
        x: color.x.sqrt(),
        y: color.y.sqrt(),
        z: color.z.sqrt(),
    }
}
