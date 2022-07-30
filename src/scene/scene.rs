use crate::scene::CameraPerspective;
use crate::scene::Sphere;
use crate::scene::Intersectable;

use crate::geo::Ray;


pub struct Scene {
    pub camera : CameraPerspective,
    pub sphere : Sphere, // TODO list of intersectable ?
}

impl Scene {
    pub fn intersect(&self, ray : &Ray) -> bool {
        return self.sphere.intersect(ray);
    }
    pub fn generate_ray(&self, x : u32, y : u32, w : u32, h : u32) -> Ray {
        return self.camera.generate_ray(x, y, w, h);
    }
}
