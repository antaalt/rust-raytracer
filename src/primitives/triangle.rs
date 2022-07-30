use std::ops::Bound;

use crate::geo::Ray;
use crate::geo::Vector2;
//use crate::geo::Vector2;
use crate::geo::Vector3;

use crate::scene::Intersectable;
use crate::scene::Intersection;
use crate::scene::Bounds;
use crate::scene::Transform;

pub struct Triangle 
{
    pub transform : Transform,
    pub a : Vector3,
    pub b : Vector3,
    pub c : Vector3,
}

impl Triangle {
    pub fn new (transform : Transform, a : Vector3, b : Vector3, c : Vector3) -> Triangle {
        return Triangle{ transform, a, b, c};
    }
}

impl Intersectable for Triangle {
    fn intersect(&self, ray: &mut Ray) -> Intersection 
    {
        let local_ray = self.transform.to_local(ray);
        // https://en.wikipedia.org/wiki/Moller–Trumbore_intersection_algorithm
        // This algorithm require ray direction to be normalized. So we normalize
        // the direction and keep the normalization factor to rescale final t.
        // This let us support intersection which where scaled by a scaled transform.
        let normalization_scale = ray.direction.length();
        let o = ray.origin;
        let d = ray.direction / normalization_scale;
        
        let ab = self.b - self.a;
        let ac = self.c - self.a;
        let h = d.cross(&ac);
        let det = ab.dot(&h);

        if det < std::f32::EPSILON // Backface culling
        {
            return Intersection::invalid();
        }

        let inv_det = 1.0 / det;
                
        let s = o - self.a;
        let u = inv_det * s.dot(&h);
        if u < 0.0 || u > 1.0
        {
            return Intersection::invalid();
        }
        let q = s.cross(&ab);
        let v = inv_det * d.dot(&q);
        if v < 0.0 || u + v > 1.0
        {
            return Intersection::invalid();
        }
        let t = inv_det * ac.dot(&q);
        if t > std::f32::EPSILON && t > local_ray.tmin && t < local_ray.tmax
        {
            let barycentric = Vector2::new(u, v);
            let hit_point = local_ray.at(t / normalization_scale);
            let normal = (self.b - self.a).cross(&(self.c - self.a)).normalize();
            let texcoord = barycentric;
            ray.tmax = t; // update closest hit
            return Intersection::new(
                self.transform.local_to_world().multiply_point(&hit_point), 
                self.transform.local_to_world().multiply_vector(&normal), 
                texcoord
            );
        }
        else
        {
            return Intersection::invalid();
        }
    }
    fn bounds(&self) -> Bounds {
        
        return Bounds::invalid();
        
    }
}