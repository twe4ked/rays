// https://raytracing.github.io/books/RayTracingInOneWeekend.html

mod camera;
mod material;
mod ppm;
mod rand;
mod ray;
mod surface;
mod vec3;
mod world;

use camera::Camera;
use rand::rand;
use vec3::Vec3;
use world::World;

use std::io;

fn main() -> io::Result<()> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 1200;
    let image_height = (image_width as f32 / aspect_ratio) as usize;

    let world = World::random_scene();

    eprintln!("Rendering image: {}x{}px...", image_width, image_height);
    let camera = init_camera(aspect_ratio);
    let colors = render_image(image_width, image_height, &camera, &world);

    eprintln!("\n\nWriting image...");
    write_image(image_width, image_height, &colors)?;

    eprintln!("Finished");

    Ok(())
}

fn init_camera(aspect_ratio: f32) -> Camera {
    let look_from = Vec3::new(13.0, 2.0, 3.0);
    let look_at = Vec3::new(0.0, 0.0, 0.0);
    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let v_fov = 20.0;
    let focus_dist = 10.0;
    let aperture = 0.1;

    Camera::new(
        look_from,
        look_at,
        v_up,
        v_fov,
        aspect_ratio,
        aperture,
        focus_dist,
    )
}

fn render_image(
    image_width: usize,
    image_height: usize,
    camera: &Camera,
    world: &World,
) -> Vec<Vec<Vec3>> {
    use rayon::prelude::*;

    let samples_per_pixel = 100;
    let max_depth = 50;

    (0..image_height)
        .map(|j| {
            if j % 100 == 0 {
                eprint!(
                    "\n{: >width$}/{} ",
                    j,
                    image_height,
                    width = image_height.to_string().len()
                );
            }
            eprint!(".");

            let j = image_height - 1 - j;
            (0..image_width)
                .into_par_iter()
                .map(|i| {
                    let mut color = Vec3::new(0.0, 0.0, 0.0);

                    for _ in 0..samples_per_pixel {
                        let u = (i as f32 + rand()) / (image_width as f32 - 1.0);
                        let v = (j as f32 + rand()) / (image_height as f32 - 1.0);
                        let ray = camera.get_ray(u, v);
                        color = color + ray.color(&world, max_depth);
                    }

                    translate_color(&color, samples_per_pixel as _)
                })
                .collect()
        })
        .collect()
}

fn write_image(image_width: usize, image_height: usize, colors: &Vec<Vec<Vec3>>) -> io::Result<()> {
    let mut stdout = io::stdout();

    ppm::write_header(&mut stdout, image_width, image_height)?;

    for row in colors.iter() {
        for color in row.iter() {
            ppm::write_color(&mut stdout, &color)?;
        }
    }

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
