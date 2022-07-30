use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Vector3 {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Vector3 {
    pub fn new(x : f32, y : f32, z : f32) -> Vector3 {
        return Vector3 { x, y, z}
    }
    pub fn distance(&self, other : &Vector3) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        return (dx * dx + dy * dy + dz * dz).sqrt()
    }
    pub fn dot(&self, other : &Vector3) -> f32 {
        return self.x * other.x + self.y * other.y + self.z * other.z;
    }
    pub fn cross(&self, other : &Vector3) -> Vector3 {
        return Vector3 {
            x : self.y * other.z - self.z * other.y,
            y : self.z * other.x - self.x * other.z,
            z : self.x * other.y - self.y * other.x,
        }
    }
    pub fn length(&self) -> f32 {
        return self.dot(self).sqrt();
    }
    pub fn normalize(&self) -> Vector3 {
        return *self / self.length();
    }
}

impl Index<usize> for Vector3 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => panic!("Index must be between 0 & 2, got {}", index),
        }
    }
}
impl IndexMut<usize> for Vector3 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            2 => &mut self.z,
            _ => panic!("Index must be between 0 & 2, got {}", index),
        }
    }
}
impl Add<Vector3> for Vector3 {
    type Output = Vector3;
    fn add(self, other : Vector3) -> Vector3 {
        return Vector3 {
            x : self.x + other.x,
            y : self.y + other.y,
            z : self.z + other.z,
        }
    }
}
impl Sub<Vector3> for Vector3 {
    type Output = Vector3;
    fn sub(self, other : Vector3) -> Vector3 {
        return Vector3 {
            x : self.x - other.x,
            y : self.y - other.y,
            z : self.z - other.z,
        }
    }
}
impl Neg for Vector3 {
    type Output = Vector3;
    fn neg(self) -> Vector3 {
        return Vector3 {
            x : -self.x,
            y : -self.y,
            z : -self.z,
        }
    }
}
impl Mul<Vector3> for Vector3 {
    type Output = Vector3;
    fn mul(self, other : Vector3) -> Vector3 {
        return Vector3 {
            x : self.x * other.x,
            y : self.y * other.y,
            z : self.z * other.z,
        }
    }
}
impl Mul<f32> for Vector3 {
    type Output = Vector3;
    fn mul(self, other : f32) -> Vector3 {
        return Vector3 {
            x : self.x * other,
            y : self.y * other,
            z : self.z * other,
        }
    }
}
impl Div<Vector3> for Vector3 {
    type Output = Vector3;
    fn div(self, other : Vector3) -> Vector3 {
        return Vector3 {
            x : self.x / other.x,
            y : self.y / other.y,
            z : self.z / other.z,
        }
    }
}
impl Div<f32> for Vector3 {
    type Output = Vector3;
    fn div(self, other : f32) -> Vector3 {
        return Vector3 {
            x : self.x / other,
            y : self.y / other,
            z : self.z / other,
        }
    }
}


#[test]
fn test_new()
{
    let p = Vector3{
        x: 1.0, 
        y: 2.0,
        z: 3.0
    };
    assert_eq!(p.x, 1.0);
    assert_eq!(p.y, 2.0);
    assert_eq!(p.z, 3.0);
    let q = Vector3::new(
        1.0, 
        2.0,
        3.0
    );
    assert_eq!(q.x, 1.0);
    assert_eq!(q.y, 2.0);
    assert_eq!(q.z, 3.0);
}