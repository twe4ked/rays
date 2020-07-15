// https://raytracing.github.io/books/RayTracingInOneWeekend.html

mod camera;
mod ppm;
mod rand;
mod ray;
mod vec3;

use camera::Camera;
use rand::Rand;
use ray::{Sphere, Surface};
use vec3::Vec3;

use std::io;

fn main() -> io::Result<()> {
    let camera = Camera::new();

    let mut stdout = io::stdout();

    ppm::write_header(
        &mut stdout,
        camera.image_width as _,
        camera.image_height as _,
    )?;

    let world: Vec<Box<dyn Surface>> = vec![
        Box::new(Sphere {
            center: Vec3::new(0.0, 0.0, -1.0),
            radius: 0.5,
        }),
        Box::new(Sphere {
            center: Vec3::new(0.0, -100.5, -1.0),
            radius: 100.0,
        }),
    ];

    let samples_per_pixel = 100;
    let max_depth = 50;

    let mut rand_ctx = Rand::new_from_time();
    let mut rand = || (rand_ctx.next() as f64 / u64::MAX as f64) as f32;

    for j in (0..(camera.image_height as usize)).rev() {
        eprint!(".");

        for i in 0..(camera.image_width as usize) {
            let mut color = Vec3::new(0.0, 0.0, 0.0);

            for _ in 0..samples_per_pixel {
                let u = (i as f32 + rand()) / (camera.image_width - 1.0);
                let v = (j as f32 + rand()) / (camera.image_height - 1.0);
                let ray = camera.get_ray(u, v);
                color = color + ray.color(&world, max_depth);
            }

            ppm::write_color(&mut stdout, &color, samples_per_pixel as _)?;
        }
    }

    eprintln!("\n");
    eprintln!("Finished");

    Ok(())
}
