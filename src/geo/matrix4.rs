use std::ops::{Mul};
use std::ops::{Index, IndexMut};

use super::Vector3;
use super::Vector4;

#[derive(Copy, Clone)]
pub struct Matrix4 {
    pub rows : [Vector4; 4]
}

impl Matrix4 {
    pub fn new(col1 : Vector4, col2 : Vector4, col3 : Vector4, col4 : Vector4) -> Matrix4 {
        return Matrix4 { rows : [col1, col2, col3, col4] }
    }
    pub fn identity() -> Matrix4
    {
        return Matrix4 { rows : 
            [
                Vector4::new(1.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 1.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 1.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 1.0),
            ]
        };
    }
    pub fn zero() -> Matrix4 
    {
        return Matrix4 { rows : 
            [
                Vector4::new(0.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 0.0),
            ]
        };
    }
    pub fn translate(x : f32, y : f32, z : f32) -> Matrix4
    {
        return Matrix4 { rows : 
            [
                Vector4::new(1.0, 0.0, 0.0, x),
                Vector4::new(0.0, 1.0, 0.0, y),
                Vector4::new(0.0, 0.0, 1.0, z),
                Vector4::new(0.0, 0.0, 0.0, 1.0),
            ]
        };
    }
    /*pub fn rotate(x : f32, y : f32, z : f32, w : f32) -> Matrix4 
    {
        return Matrix4 { rows : 
            [
                Vector4::new(0.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 0.0),
            ]
        };
    }*/
    pub fn scale(x : f32, y : f32, z : f32) -> Matrix4 
    {
        return Matrix4 { rows : 
            [
                Vector4::new(x, 0.0, 0.0, 0.0),
                Vector4::new(0.0, y, 0.0, 0.0),
                Vector4::new(0.0, 0.0, z, 0.0),
                Vector4::new(0.0, 0.0, 0.0, 1.0),
            ]
        };
    }
    pub fn transpose(&self) -> Matrix4 
    {
        return Matrix4 { rows : 
            [
                Vector4::new(self.rows[0].x, self.rows[1].x, self.rows[2].x, self.rows[3].x),
                Vector4::new(self.rows[0].y, self.rows[1].y, self.rows[2].y, self.rows[3].y),
                Vector4::new(self.rows[0].z, self.rows[1].z, self.rows[2].z, self.rows[3].z),
                Vector4::new(self.rows[0].w, self.rows[1].w, self.rows[2].w, self.rows[3].w),
            ]
        };
    }
    pub fn determinant(&self) -> f32 
    {
        return
            self.rows[3][0] * self.rows[2][1] * self.rows[1][2] * self.rows[0][3] - self.rows[2][0] * self.rows[3][1] * self.rows[1][2] * self.rows[0][3] -
            self.rows[3][0] * self.rows[1][1] * self.rows[2][2] * self.rows[0][3] + self.rows[1][0] * self.rows[3][1] * self.rows[2][2] * self.rows[0][3] +
            self.rows[2][0] * self.rows[1][1] * self.rows[3][2] * self.rows[0][3] - self.rows[1][0] * self.rows[2][1] * self.rows[3][2] * self.rows[0][3] -
            self.rows[3][0] * self.rows[2][1] * self.rows[0][2] * self.rows[1][3] + self.rows[2][0] * self.rows[3][1] * self.rows[0][2] * self.rows[1][3] +
            self.rows[3][0] * self.rows[0][1] * self.rows[2][2] * self.rows[1][3] - self.rows[0][0] * self.rows[3][1] * self.rows[2][2] * self.rows[1][3] -
            self.rows[2][0] * self.rows[0][1] * self.rows[3][2] * self.rows[1][3] + self.rows[0][0] * self.rows[2][1] * self.rows[3][2] * self.rows[1][3] +
            self.rows[3][0] * self.rows[1][1] * self.rows[0][2] * self.rows[2][3] - self.rows[1][0] * self.rows[3][1] * self.rows[0][2] * self.rows[2][3] -
            self.rows[3][0] * self.rows[0][1] * self.rows[1][2] * self.rows[2][3] + self.rows[0][0] * self.rows[3][1] * self.rows[1][2] * self.rows[2][3] +
            self.rows[1][0] * self.rows[0][1] * self.rows[3][2] * self.rows[2][3] - self.rows[0][0] * self.rows[1][1] * self.rows[3][2] * self.rows[2][3] -
            self.rows[2][0] * self.rows[1][1] * self.rows[0][2] * self.rows[3][3] + self.rows[1][0] * self.rows[2][1] * self.rows[0][2] * self.rows[3][3] +
            self.rows[2][0] * self.rows[0][1] * self.rows[1][2] * self.rows[3][3] - self.rows[0][0] * self.rows[2][1] * self.rows[1][2] * self.rows[3][3] -
            self.rows[1][0] * self.rows[0][1] * self.rows[2][2] * self.rows[3][3] + self.rows[0][0] * self.rows[1][1] * self.rows[2][2] * self.rows[3][3];
    }
    pub fn inverse(&self) -> Matrix4 
    {
        let a2323 = self.rows[2][2] * self.rows[3][3] - self.rows[3][2] * self.rows[2][3];
        let a1323 = self.rows[1][2] * self.rows[3][3] - self.rows[3][2] * self.rows[1][3];
        let a1223 = self.rows[1][2] * self.rows[2][3] - self.rows[2][2] * self.rows[1][3];
        let a0323 = self.rows[0][2] * self.rows[3][3] - self.rows[3][2] * self.rows[0][3];
        let a0223 = self.rows[0][2] * self.rows[2][3] - self.rows[2][2] * self.rows[0][3];
        let a0123 = self.rows[0][2] * self.rows[1][3] - self.rows[1][2] * self.rows[0][3];
        let a2313 = self.rows[2][1] * self.rows[3][3] - self.rows[3][1] * self.rows[2][3];
        let a1313 = self.rows[1][1] * self.rows[3][3] - self.rows[3][1] * self.rows[1][3];
        let a1213 = self.rows[1][1] * self.rows[2][3] - self.rows[2][1] * self.rows[1][3];
        let a2312 = self.rows[2][1] * self.rows[3][2] - self.rows[3][1] * self.rows[2][2];
        let a1312 = self.rows[1][1] * self.rows[3][2] - self.rows[3][1] * self.rows[1][2];
        let a1212 = self.rows[1][1] * self.rows[2][2] - self.rows[2][1] * self.rows[1][2];
        let a0313 = self.rows[0][1] * self.rows[3][3] - self.rows[3][1] * self.rows[0][3];
        let a0213 = self.rows[0][1] * self.rows[2][3] - self.rows[2][1] * self.rows[0][3];
        let a0312 = self.rows[0][1] * self.rows[3][2] - self.rows[3][1] * self.rows[0][2];
        let a0212 = self.rows[0][1] * self.rows[2][2] - self.rows[2][1] * self.rows[0][2];
        let a0113 = self.rows[0][1] * self.rows[1][3] - self.rows[1][1] * self.rows[0][3];
        let a0112 = self.rows[0][1] * self.rows[1][2] - self.rows[1][1] * self.rows[0][2];

        let det = 1.0 / self.determinant();

        return Matrix4 { rows : 
            [
                Vector4::new(
                    det *  (self.rows[1][1] * a2323 - self.rows[2][1] * a1323 + self.rows[3][1] * a1223),
                    det * -(self.rows[1][0] * a2323 - self.rows[2][0] * a1323 + self.rows[3][0] * a1223),
                    det *  (self.rows[1][0] * a2313 - self.rows[2][0] * a1313 + self.rows[3][0] * a1213),
                    det * -(self.rows[1][0] * a2312 - self.rows[2][0] * a1312 + self.rows[3][0] * a1212)
                ),
                Vector4::new(
                    det * -(self.rows[0][1] * a2323 - self.rows[2][1] * a0323 + self.rows[3][1] * a0223),
                    det *  (self.rows[0][0] * a2323 - self.rows[2][0] * a0323 + self.rows[3][0] * a0223),
                    det * -(self.rows[0][0] * a2313 - self.rows[2][0] * a0313 + self.rows[3][0] * a0213),
                    det *  (self.rows[0][0] * a2312 - self.rows[2][0] * a0312 + self.rows[3][0] * a0212)
                ),
                Vector4::new(
                    det *  (self.rows[0][1] * a1323 - self.rows[1][1] * a0323 + self.rows[3][1] * a0123),
                    det * -(self.rows[0][0] * a1323 - self.rows[1][0] * a0323 + self.rows[3][0] * a0123),
                    det *  (self.rows[0][0] * a1313 - self.rows[1][0] * a0313 + self.rows[3][0] * a0113),
                    det * -(self.rows[0][0] * a1312 - self.rows[1][0] * a0312 + self.rows[3][0] * a0112)
                ),
                Vector4::new(
                    det * -(self.rows[0][1] * a1223 - self.rows[1][1] * a0223 + self.rows[2][1] * a0123),
                    det *  (self.rows[0][0] * a1223 - self.rows[1][0] * a0223 + self.rows[2][0] * a0123),
                    det * -(self.rows[0][0] * a1213 - self.rows[1][0] * a0213 + self.rows[2][0] * a0113),
                    det *  (self.rows[0][0] * a1212 - self.rows[1][0] * a0212 + self.rows[2][0] * a0112)
                ),
            ]
        }.transpose();
    }
    pub fn look_at(eye : Vector3, target : Vector3, up : Vector3) -> Matrix4
    {
        // Right handed lookAt
        let forward : Vector3 = (target - eye).normalize();
        let right : Vector3 = forward.cross(&up).normalize();
        let up_coordinate : Vector3 = right.cross(&forward).normalize();
    
        return Matrix4::new(
            Vector4::new(right.x, up_coordinate.x, -forward.x, eye.x),
            Vector4::new(right.y, up_coordinate.y, -forward.y, eye.y),
            Vector4::new(right.z, up_coordinate.z, -forward.z, eye.z),
            Vector4::new(0.0, 0.0, 0.0, 1.0),
        );
    }
    pub fn perspective(fov_y : f32, ratio : f32, near : f32, far : f32) -> Matrix4 
    {
        // Right handed, clip space positive.
        let f = 1.0 / (fov_y / 2.0).tan();
        return Matrix4::new(
            Vector4::new(f / ratio, 0.0, 0.0, 0.0),
            Vector4::new(0.0, f, 0.0, 0.0),
            Vector4::new(0.0, 0.0, far / (near - far), -1.0),
            Vector4::new(0.0, 0.0, -(far * near) / (far - near), 0.0)
        ).transpose();
    }
    pub fn multiply_point(&self, point : &Vector3) -> Vector3 
    {
        let w : f32 = self.rows[3].x * point.x + self.rows[3].y * point.y + self.rows[3].z * point.z + self.rows[3].w;
        return Vector3::new(
            (self.rows[0].x * point.x + self.rows[0].y * point.y + self.rows[0].z * point.z + self.rows[0].w) / w,
            (self.rows[1].x * point.x + self.rows[1].y * point.y + self.rows[1].z * point.z + self.rows[1].w) / w,
            (self.rows[2].x * point.x + self.rows[2].y * point.y + self.rows[2].z * point.z + self.rows[2].w) / w,
        );
    }
    pub fn multiply_vector(&self, point : &Vector3) -> Vector3 
    {
        return Vector3::new(
            self.rows[0].x * point.x + self.rows[0].y * point.y + self.rows[0].z * point.z,
            self.rows[1].x * point.x + self.rows[1].y * point.y + self.rows[1].z * point.z,
            self.rows[2].x * point.x + self.rows[2].y * point.y + self.rows[2].z * point.z,
        );
    }
}

impl Index<usize> for Matrix4 {
    type Output = Vector4;
    fn index(&self, i: usize) -> &Vector4 {
        &self.rows[i]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(&mut self, i: usize) -> &mut Vector4 {
        &mut self.rows[i]
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;
    fn mul(self, other : Matrix4) -> Matrix4 {
        let mut out = Matrix4::zero();
        // TODO simplify operations.
        for i_col in 0..4
        {
            for i_row in 0..4
            {
                for k in 0..4
                {
                    out[i_row][i_col] += self.rows[i_row][k] * other.rows[k][i_col];
                }
            }
        }
        return out;
    }
}
impl Mul<Vector4> for Matrix4 {
    type Output = Vector4;
    fn mul(self, other : Vector4) -> Vector4 {
        return Vector4::new(
            self.rows[0].x * other.x + self.rows[1].x * other.y + self.rows[2].x * other.z + self.rows[3].x * other.w,
            self.rows[0].y * other.x + self.rows[1].y * other.y + self.rows[2].y * other.z + self.rows[3].y * other.w,
            self.rows[0].z * other.x + self.rows[1].z * other.y + self.rows[2].z * other.z + self.rows[3].z * other.w,
            self.rows[0].w * other.x + self.rows[1].w * other.y + self.rows[2].w * other.z + self.rows[3].w * other.w
        );
    }
}