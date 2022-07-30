use crate::geo::Ray;
use crate::geo::Vector2;
use crate::geo::Vector3;
use crate::geo::Matrix4;

// Camera trait, derive CameraFisheye, CameraParallel, CameraODS ?

// TODO inherit transform
pub struct CameraPerspective {
    pub transform : super::Transform,
    pub fov : f32,
    pub ratio : f32
}

impl CameraPerspective {
    pub fn generate_ray(&self, x : u32, y : u32, w : u32, h : u32) -> Ray {

        let screen_pos = Vector2::new((x as f32 / w as f32) * 2.0 - 1.0, (y as f32 / h as f32) * 2.0 - 1.0);
        let projection = Matrix4::perspective(self.fov / 180.0 * std::f32::consts::PI, w as f32 / h as f32, 0.1, 1000.0);

        let view_inverse = self.transform.matrix();
        let proj_inverse = projection.inverse();
        
        let cam_pos      = view_inverse.multiply_point(&Vector3::new(0.0, 0.0, 0.0));
        let cam_target   = proj_inverse.multiply_point(&Vector3::new(screen_pos.x, screen_pos.y, 0.0));
        let cam_dir      = view_inverse.multiply_vector(&cam_target.normalize()); // TODO check 0,0 = nan

        return Ray::new(
            cam_pos, 
            cam_dir.normalize()
        );
    }

    /*pub fn view() {

    }
    pub fn transform() {

    }*/
}