use crate::ray::Ray;
use crate::vec3::Vec3;

pub struct Camera {
    origin: Vec3,
    horizontal: Vec3,
    vertical: Vec3,
    focal_length: f32,
}

impl Camera {
    pub fn new(v_fov: f32, aspect_ratio: f32) -> Self {
        let theta = v_fov.to_radians();
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let focal_length = 1.0;

        let origin = Vec3::new(0.0, 0.0, 0.0);
        let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
        let vertical = Vec3::new(0.0, viewport_height, 0.0);

        Self {
            origin,
            horizontal,
            vertical,
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
