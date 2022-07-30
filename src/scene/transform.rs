use crate::geo::Vector3;
use crate::geo::Matrix4;

pub struct Transform {
    matrix : Matrix4,
}

impl Transform {
    pub fn identity() -> Transform {
        return Transform{ matrix : Matrix4::identity() };
    }
    pub fn new(matrix : Matrix4) -> Transform {
        return Transform{ matrix : matrix };
    }
    pub fn matrix(&self) -> Matrix4 {
        return self.matrix;
    }
    pub fn position(&self) -> Vector3 {
        return Vector3::new(self.matrix.rows[0].w, self.matrix.rows[1].w, self.matrix.rows[2].w)
    }
    pub fn scale(&self) -> Vector3 {
        return Vector3::new(self.matrix.rows[0].x, self.matrix.rows[1].y, self.matrix.rows[2].z)
    }
    /*pub fn rotation(&self) -> Vector3 {
        return Vector3::new(self.matrix.rows[0].w, self.matrix.rows[1].w, self.matrix.rows[2].w)
    }*/
}