// https://raytracing.github.io/books/RayTracingInOneWeekend.html

mod camera;
mod ppm;
mod ray;
mod vec3;

use camera::Camera;
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

    for j in (0..(camera.image_height as usize)).rev() {
        eprint!(".");

        for i in 0..(camera.image_width as usize) {
            let u = i as f32 / (camera.image_width - 1.0);
            let v = j as f32 / (camera.image_height - 1.0);
            let ray = camera.get_ray(u, v);

            ppm::write_color(&mut stdout, &ray.color(&world))?;
        }
    }

    eprintln!("\n");
    eprintln!("Finished");

    Ok(())
}
