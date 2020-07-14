// https://raytracing.github.io/books/RayTracingInOneWeekend.html

mod ppm;
mod ray;
mod vec3;

use ray::{Ray, Sphere, Surface};
use vec3::Vec3;

use std::io;

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 384;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, focal_length);

    let mut stdout = io::stdout();

    ppm::write_header(&mut stdout, image_width, image_height)?;

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

    for j in (0..image_height).rev() {
        eprint!(".");

        for i in 0..image_width {
            let u = i as f32 / (image_width as f32 - 1.0);
            let v = j as f32 / (image_height as f32 - 1.0);
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );

            ppm::write_color(&mut stdout, &ray.color(&world))?;
        }
    }

    eprintln!("\n");
    eprintln!("Finished");

    Ok(())
}
