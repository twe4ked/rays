// https://en.wikipedia.org/wiki/Netpbm_format#PPM_example

use crate::vec3::Vec3;

use std::io::{self, Write};

pub fn write_color(output: &mut dyn Write, color: &Vec3) -> io::Result<()> {
    let r = (255.0 * color.x) as usize;
    let g = (255.0 * color.y) as usize;
    let b = (255.0 * color.z) as usize;

    writeln!(output, "{} {} {}", r, g, b)
}

pub fn write_header(output: &mut dyn Write, width: usize, height: usize) -> io::Result<()> {
    writeln!(output, "P3\n{} {}\n255", width, height)
}
