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
    let image_width = 384;
    let image_height = 216;
    let camera = Camera::new(90.0, image_width as f32 / image_height as f32);

    let mut stdout = io::stdout();

    ppm::write_header(&mut stdout, image_width, image_height)?;

    let world: Vec<(Box<dyn Surface>, Box<dyn Material>)> = vec![
        (
            Box::new(Sphere::new(Vec3::new(0.0, 0.0, -1.0), 0.5)),
            Box::new(Lambertian::new(Vec3::new(0.1, 0.2, 0.5))),
        ),
        (
            Box::new(Sphere::new(Vec3::new(0.0, -100.5, -1.0), 100.0)),
            Box::new(Lambertian::new(Vec3::new(0.8, 0.8, 0.0))),
        ),
        (
            Box::new(Sphere::new(Vec3::new(1.0, 0.0, -1.0), 0.5)),
            Box::new(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3)),
        ),
        (
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), 0.5)),
            Box::new(Dielectric::new(1.5)),
        ),
        (
            Box::new(Sphere::new(Vec3::new(-1.0, 0.0, -1.0), -0.45)),
            Box::new(Dielectric::new(1.5)),
        ),
    ];

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
