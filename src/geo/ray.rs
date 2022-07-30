use super::Vector3;

#[derive(Copy, Clone)]
pub struct Ray {
    pub origin : Vector3,
    pub direction : Vector3,
    pub tmin : f32,
    pub tmax : f32
}


impl Ray {
    pub fn new(origin : Vector3, direction : Vector3) -> Ray {
        Ray { origin, direction, tmin : 0.1, tmax : 1000.0 }
    }
    pub fn at(&self, t : f32) -> Vector3 {
        self.origin + self.direction * t
    }
}