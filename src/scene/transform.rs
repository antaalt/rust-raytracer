use crate::geo::Vector3;
use crate::geo::Matrix4;
use crate::geo::Ray;

pub struct Transform {
    local_to_world : Matrix4,
    world_to_local : Matrix4,
}

impl Transform {
    pub fn identity() -> Transform {
        return Transform{ local_to_world : Matrix4::identity(), world_to_local : Matrix4::identity() };
    }
    pub fn new(matrix : Matrix4) -> Transform {
        return Transform{ local_to_world : matrix, world_to_local : matrix.inverse() };
    }
    pub fn local_to_world(&self) -> Matrix4 {
        return self.local_to_world;
    }
    pub fn world_to_local(&self) -> Matrix4 {
        return self.local_to_world;
    }
    pub fn position(&self) -> Vector3 {
        return Vector3::new(self.local_to_world.rows[0].w, self.local_to_world.rows[1].w, self.local_to_world.rows[2].w)
    }
    pub fn scale(&self) -> Vector3 {
        return Vector3::new(self.local_to_world.rows[0].x, self.local_to_world.rows[1].y, self.local_to_world.rows[2].z)
    }
    /*pub fn rotation(&self) -> Vector3 {
        return Vector3::new(self.matrix.rows[0].w, self.matrix.rows[1].w, self.matrix.rows[2].w)
    }*/

    fn transform_ray(ray : &Ray, transform : &Matrix4) -> Ray 
    {
        let min_pos = ray.at(ray.tmin);
        let max_pos = ray.at(ray.tmax);
        let mut out = ray.clone();
        out.origin = transform.multiply_point(&out.origin);
        out.direction = transform.multiply_vector(&out.direction);
        if out.tmax < std::f32::MAX
        {
            out.tmin = out.origin.distance(&transform.multiply_point(&min_pos));
            out.tmax = out.origin.distance(&transform.multiply_point(&max_pos));
        }
        return out;
    }

    pub fn to_world(&self, ray : &Ray) -> Ray
    {
        return Transform::transform_ray(ray, &self.local_to_world);
    }

    pub fn to_local(&self, ray : &Ray) -> Ray
    {
        return Transform::transform_ray(ray, &self.world_to_local);
    }
}