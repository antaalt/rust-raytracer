use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{Index, IndexMut};

use super::Vector3;

#[derive(Copy, Clone)]
pub struct Vector4 {
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub w : f32,
}

impl Vector4 {
    pub fn new(x : f32, y : f32, z : f32, w : f32) -> Vector4 {
        return Vector4 { x, y, z, w }
    }
    pub fn zero() -> Vector4 {
        return Vector4 { x: 0.0, y: 0.0, z: 0.0, w: 0.0 }
    }
    pub fn from_vector3(vec : &Vector3, w : f32) -> Vector4 {
        return Vector4::new(vec.x, vec.y, vec.z, w)
    }
}

impl Index<usize> for Vector4 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            3 => &self.w,
            _ => panic!("Index must be between 0 & 3, got {}", index),
        }
    }
}
impl IndexMut<usize> for Vector4 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            3 => &mut self.w,
            _ => panic!("Index must be between 0 & 3, got {}", index),
        }
    }
}
impl Add<Vector4> for Vector4 {
    type Output = Vector4;
    fn add(self, other : Vector4) -> Vector4 {
        return Vector4 {
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z,
            w : self.w + other.w,
        }
    }
}
impl Sub<Vector4> for Vector4 {
    type Output = Vector4;
    fn sub(self, other : Vector4) -> Vector4 {
        return Vector4 {
            x : self.x - other.x,
            y : self.y - other.y,
            z : self.z - other.z,
            w : self.w - other.w,
        }
    }
}
impl Neg for Vector4 {
    type Output = Vector4;
    fn neg(self) -> Vector4 {
        return Vector4 {
            x : -self.x,
            y : -self.y,
            z : -self.z,
            w : -self.w,
        }
    }
}
impl Mul<Vector4> for Vector4 {
    type Output = Vector4;
    fn mul(self, other : Vector4) -> Vector4 {
        return Vector4 {
            x : self.x * other.x,
            y : self.y * other.y,
            z : self.z * other.z,
            w : self.w * other.w,
        }
    }
}
impl Mul<f32> for Vector4 {
    type Output = Vector4;
    fn mul(self, other : f32) -> Vector4 {
        return Vector4 {
            x : self.x * other,
            y : self.y * other,
            z : self.z * other,
            w : self.w * other,
        }
    }
}
impl Div<Vector4> for Vector4 {
    type Output = Vector4;
    fn div(self, other : Vector4) -> Vector4 {
        return Vector4 {
            x : self.x / other.x,
            y : self.y / other.y,
            z : self.z / other.z,
            w : self.w / other.w,
        }
    }
}
impl Div<f32> for Vector4 {
    type Output = Vector4;
    fn div(self, other : f32) -> Vector4 {
        return Vector4 {
            x : self.x / other,
            y : self.y / other,
            z : self.z / other,
            w : self.w / other,
        }
    }
}


#[test]
fn test_new()
{
    let p = Vector4{
        x: 1.0, 
        y: 2.0,
        z: 3.0,
        w: 4.0,
    };
    assert_eq!(p.x, 1.0);
    assert_eq!(p.y, 2.0);
    assert_eq!(p.z, 3.0);
    assert_eq!(p.w, 4.0);
    let q = Vector4::new(
        1.0, 
        2.0,
        3.0,
        4.0,
    );
    assert_eq!(q.x, 1.0);
    assert_eq!(q.y, 2.0);
    assert_eq!(q.z, 3.0);
    assert_eq!(q.w, 4.0);
}