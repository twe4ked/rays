use crate::material::Material;
use crate::surface::Surface;

pub struct World {
    pub objects: Vec<(Box<dyn Surface + Sync>, Box<dyn Material + Sync>)>,
}

impl World {
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, surface: Box<dyn Surface + Sync>, material: Box<dyn Material + Sync>) {
        self.objects.push((surface, material));
    }
}
