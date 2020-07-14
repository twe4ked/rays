use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    pub image_width: f32,
    pub image_height: f32,
    focal_length: f32,
}

impl Camera {
    pub fn new() -> Self {
        let aspect_ratio = 16.0 / 9.0;
        let image_width = 384.0;
        let image_height = image_width / aspect_ratio;

        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Self {
            origin,
            horizontal,
            vertical,
            image_width,
            image_height,
            focal_length,
        }
    }

    pub fn get_ray(&self, u: f32, v: f32) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner() + u * self.horizontal + v * self.vertical - self.origin,
        )
    }

    fn lower_left_corner(&self) -> Vec3 {
        self.origin
            - self.horizontal / 2.0
            - self.vertical / 2.0
            - Vec3::new(0.0, 0.0, self.focal_length)
    }
}
