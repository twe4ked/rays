use crate::material::Material;
use crate::surface::Surface;

pub struct World {
    pub objects: Vec<(Box<dyn Surface>, Box<dyn Material>)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, surface: Box<dyn Surface>, material: Box<dyn Material>) {
        self.objects.push((surface, material));
    }
}
