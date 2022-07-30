use crate::geo::Ray;
//use crate::geo::Vector2;
use crate::geo::Vector3;

use crate::scene::Intersectable;

// TODO inherit mesh
pub struct Sphere {
    pub position : Vector3,
    pub radius : f32,
}

impl Sphere {
    pub fn new (position : Vector3, radius : f32) -> Sphere {
        return Sphere{ position : position, radius : radius };
    }
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> bool {
        // World to local transform
        let o : &Vector3 = &ray.origin;
        let d : &Vector3 = &ray.direction;
        
        let a : f32 = d.dot(d);
        let b : f32 = 2.0 * d.dot(o);
        let c : f32 = o.dot(o) - self.radius * self.radius;
        let discriminant : f32 = b * b - 4.0 * a * c;

        if discriminant < 0.0 {
            return false;
        }
        
        let delta : f32 = discriminant.sqrt();

        let t1 = (-b - delta) / (2.0 * a);
        let t2 = (-b + delta) / (2.0 * a);
        // Local to world transform
        if t1 > ray.tmin && t1 < ray.tmax
        {
            // Local to world transform
            let center = Vector3::new(0.0, 0.0, 0.0);
            let mut hit_point = ray.at(t1);
            hit_point = hit_point * self.radius / hit_point.distance(&center); // Refine hit
            let normal : Vector3 = (hit_point - center).normalize();
            // https://en.wikipedia.org/wiki/UV_mapping
            let u = 0.5 + (-normal.z.atan2(-normal.x) / (2.0 * std::f32::consts::PI));
            let v = 0.5 - (-normal.y.asin() / std::f32::consts::PI);
            //let texcoord = Vector2::new(u, v);
            //ray.tmax = t1;
            return true;

        }
        else if t2 > ray.tmin && t2 < ray.tmax
        {
            // Local to world transform
            let center = Vector3::new(0.0, 0.0, 0.0);
            let mut hit_point = ray.at(t2);
            hit_point = hit_point * self.radius / hit_point.distance(&center); // Refine hit
            let normal : Vector3 = (hit_point - center).normalize();
            // https://en.wikipedia.org/wiki/UV_mapping
            let u = 0.5 + (-normal.z.atan2(-normal.x) / (2.0 * std::f32::consts::PI));
            let v = 0.5 - (-normal.y.asin() / std::f32::consts::PI);
           // let texcoord = Vector2::new(u, v);
           // ray.tmax = t2;
            return true;
        }
        else 
        {
            return false;
        }
    }
}