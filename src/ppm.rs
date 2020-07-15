// https://en.wikipedia.org/wiki/Netpbm_format#PPM_example

use crate::vec3::Vec3;

use std::io::{self, Write};

pub fn write_color(output: &mut dyn Write, color: &Vec3, samples_per_pixel: f32) -> io::Result<()> {
    let scale = 1.0 / samples_per_pixel;

    let r = color.x * scale;
    let g = color.y * scale;
    let b = color.z * scale;

    let r = (256.0 * clamp(r, 0.0, 0.999)) as usize;
    let g = (256.0 * clamp(g, 0.0, 0.999)) as usize;
    let b = (256.0 * clamp(b, 0.0, 0.999)) as usize;

    writeln!(output, "{} {} {}", r, g, b)
}

pub fn write_header(output: &mut dyn Write, width: usize, height: usize) -> io::Result<()> {
    writeln!(output, "P3\n{} {}\n255", width, height)
}

fn clamp(x: f32, min: f32, max: f32) -> f32 {
    if x < min {
        return min;
    };
    if x > max {
        return max;
    };
    x
}
