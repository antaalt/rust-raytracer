use crate::geo::Ray;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> bool;
}
