use std::ops::{Add, Div, Mul, Neg, Sub};
use std::ops::{Index, IndexMut};

#[derive(Copy, Clone)]
pub struct Vector2 {
    pub x : f32,
    pub y : f32,
}

impl Vector2 {
    pub fn new(x : f32, y : f32) -> Vector2 {
        return Vector2 { x, y }
    }
    pub fn distance(&self, other : &Vector2) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        return (dx * dx + dy * dy).sqrt()
    }
    pub fn dot(&self, other : &Vector2) -> f32 {
        return self.x * other.x + self.y * other.y;
    }
    pub fn length(&self) -> f32 {
        return self.dot(self).sqrt();
    }
}

impl Index<usize> for Vector2 {
    type Output = f32;
    fn index(&self, index: usize) -> &f32 {
        match index {
            0 => &self.x,
            1 => &self.y,
            _ => panic!("Index must be between 0 & 1, got {}", index),
        }
    }
}
impl IndexMut<usize> for Vector2 {
    fn index_mut(&mut self, index: usize) -> &mut f32 {
        match index {
            0 => &mut self.x,
            1 => &mut self.y,
            _ => panic!("Index must be between 0 & 1, got {}", index),
        }
    }
}
impl Add<Vector2> for Vector2 {
    type Output = Vector2;
    fn add(self, other : Vector2) -> Vector2 {
        return Vector2 {
            x : self.x + other.x,
            y : self.y + other.y,
        }
    }
}
impl Sub<Vector2> for Vector2 {
    type Output = Vector2;
    fn sub(self, other : Vector2) -> Vector2 {
        return Vector2 {
            x : self.x - other.x,
            y : self.y - other.y,
        }
    }
}
impl Neg for Vector2 {
    type Output = Vector2;
    fn neg(self) -> Vector2 {
        return Vector2 {
            x : -self.x,
            y : -self.y,
        }
    }
}
impl Mul<Vector2> for Vector2 {
    type Output = Vector2;
    fn mul(self, other : Vector2) -> Vector2 {
        return Vector2 {
            x : self.x * other.x,
            y : self.y * other.y,
        }
    }
}
impl Mul<f32> for Vector2 {
    type Output = Vector2;
    fn mul(self, other : f32) -> Vector2 {
        return Vector2 {
            x : self.x * other,
            y : self.y * other,
        }
    }
}
impl Div<Vector2> for Vector2 {
    type Output = Vector2;
    fn div(self, other : Vector2) -> Vector2 {
        return Vector2 {
            x : self.x / other.x,
            y : self.y / other.y,
        }
    }
}
impl Div<f32> for Vector2 {
    type Output = Vector2;
    fn div(self, other : f32) -> Vector2 {
        return Vector2 {
            x : self.x / other,
            y : self.y / other,
        }
    }
}


#[test]
fn test_new()
{
    let p = Vector2{
        x: 1.0, 
        y: 2.0,
    };
    assert_eq!(p.x, 1.0);
    assert_eq!(p.y, 2.0);
    let q = Vector2::new(
        1.0, 
        2.0,
    );
    assert_eq!(q.x, 1.0);
    assert_eq!(q.y, 2.0);
}