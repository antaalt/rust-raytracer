use crate::scene::CameraPerspective;
use crate::scene::Intersectable;
use crate::scene::Intersection;
use crate::scene::Transform;

use crate::primitives::Sphere;
use crate::primitives::Triangle;

use crate::geo::Matrix4;
use crate::geo::Ray;
use crate::geo::Vector3;

use std::path::Path;
use std::vec::Vec;


pub struct Scene {
    camera : CameraPerspective,
    intersectables : Vec<Box<dyn Intersectable>>,
}

impl Scene {
    pub fn new() -> Scene
    {
        let mut scene = Scene {
            camera: CameraPerspective {
                transform : Transform::new(Matrix4::look_at(
                    Vector3::new(0.0, 0.0, 10.0),
                    Vector3::new(0.0, 0.0, 0.0),
                    Vector3::new(0.0, 1.0, 0.0),
                )),
                fov : 90.0,
                ratio : 1.0
            },
            intersectables: Vec::new()
        };
        scene.intersectables.push(Box::new(Sphere::new( 
            Transform::new(Matrix4::translate(0.0, 0.0, 0.0)), 
            2.0 
        )));
        scene.intersectables.push(Box::new(Sphere::new( 
            Transform::new(Matrix4::translate(0.0, 4.0, 4.0)), 
            2.0 
        )));
        scene.intersectables.push(Box::new(Sphere::new( 
            Transform::new(Matrix4::translate(4.0, 0.0, 0.0)), 
            2.0 
        )));
        scene.intersectables.push(Box::new(Triangle::new( 
            Transform::new(Matrix4::translate(4.0, 0.0, 0.0)), 
            Vector3::new(0.0, 0.0, 0.0),
            Vector3::new(5.0, 0.0, 1.0),
            Vector3::new(0.0, 5.0, 1.0),
        )));
        return scene;
    }
    pub fn save<P: AsRef<Path>>(&self, path : &P)
    {
        // TODO save to JSON
    }
    pub fn load<P: AsRef<Path>>(&self, path : &P)
    {
        // TODO load from JSON
    }
    pub fn intersect(&self, _ray : Ray) -> Intersection {
        let mut ray : Ray = _ray.clone();
        let mut final_intersection = Intersection::invalid();
        for sphere in self.intersectables.iter() {
            let intersection = sphere.intersect(&mut ray);
            if intersection.valid()
            {
                final_intersection = intersection;
            }
        }
        return final_intersection;
    }
    pub fn generate_ray(&self, x : u32, y : u32, w : u32, h : u32) -> Ray {
        return self.camera.generate_ray(x, y, w, h);
    }
}
