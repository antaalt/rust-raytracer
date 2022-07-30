use super::Vector3;


pub struct Quaternion {
    pub x : f32,
    pub y : f32,
    pub z : f32,
    pub w : f32,
}
impl Quaternion {
    pub fn new(x : f32, y : f32, z : f32, w : f32) -> Quaternion 
    {
        return Quaternion { x: x, y: y, z: z, w: w };
    }
    pub fn axis(axis : Vector3, angle : f32) -> Quaternion
    {
        let sin_half_angle = (angle / 2.0).sin();
        return Quaternion::new(
            axis.x * sin_half_angle,
            axis.y * sin_half_angle,
            axis.z * sin_half_angle,
            (angle / 2.0).cos()
        );
    }
    pub fn euler(yaw : f32, pitch : f32, roll : f32) -> Quaternion
    {
        let c1 = (yaw / 2.0).cos();
        let s1 = (yaw / 2.0).sin();
        let c2 = (pitch / 2.0).cos();
        let s2 = (pitch / 2.0).sin();
        let c3 = (roll / 2.0).cos();
        let s3 = (roll / 2.0).sin();
        return Quaternion::new(
            s1 * c2 * c3 + c1 * s2 * s3,
            c1 * s2 * c3 - s1 * c2 * s3,
            c1 * c2 * s3 - s1 * s2 * c3,
            c1 * c2 * c3 + s1 * s2 * s3
        );
    }
    pub fn identity() -> Quaternion 
    {
        return Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 };
    }
    pub fn conjugate(&self) -> Quaternion 
    {
        return Quaternion { x: -self.x, y: -self.y, z: -self.z, w: self.w };
    }

}