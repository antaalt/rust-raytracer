use crate::geo::Color;
use crate::geo::Ray;

use crate::geo::Vector2;
use crate::geo::Vector3;

#[derive(Copy, Clone)]
pub struct Intersection
{
    pub point : Vector3,
    pub normal : Vector3,
    pub texcoord : Vector2,

    valid : bool,
}
pub struct Bounds
{
    min : Vector3,
    max : Vector3,
}

impl Intersection 
{
    pub fn new(point : Vector3, normal : Vector3, texcoord : Vector2) -> Intersection
    {
        return Intersection { point, normal, texcoord, valid: true };
    }
    pub fn invalid() -> Intersection
    {
        return Intersection { point: Vector3::zero(), normal: Vector3::zero(), texcoord: Vector2::zero(), valid: false };
    }
    pub fn valid(&self) -> bool
    {
        return self.valid
    }

    pub fn shade(&self) -> Color
    {
        let costheta = self.normal.dot(&Vector3::new(0.0, 1.0, 0.0));
        let value : f32 = costheta * 0.5 + 0.5;
        return Color::new(value, value, value, 1.0);
    }
}

impl Bounds 
{
    pub fn new(min : Vector3, max : Vector3) -> Bounds
    {
        return Bounds { min, max };
    }
    pub fn invalid() -> Bounds
    {
        return Bounds::new(Vector3::zero(), Vector3::zero());
    }
    pub fn intersect(ray : &Ray) -> bool
    {
        return false;
    }
}

pub trait Intersectable 
{
    fn intersect(&self, ray: &mut Ray) -> Intersection;

    fn bounds(&self) -> Bounds;
}
