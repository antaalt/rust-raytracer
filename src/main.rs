use image::{ImageBuffer };

mod geo;
use geo::Vector3;

mod scene;
use scene::Scene;
use scene::Sphere;
use scene::CameraPerspective;

use crate::geo::Matrix4;
use crate::scene::Transform;


fn main() {
    let width : u32 = 800;
    let height : u32 = 600;
    let scene = Scene {
        camera: CameraPerspective {
            transform : Transform::new(Matrix4::look_at(
                Vector3::new(0.0, 0.0, 5.0),
                Vector3::new(0.0, 0.0, 0.0),
                Vector3::new(0.0, 1.0, 0.0),
            )),
            fov : 90.0,
            ratio : 1.0
        },
        sphere : Sphere::new( 
            Vector3 { x : 0.0, y : 0.0, z : 0.0 }, 
            2.0 
        )
    };
    /*let img = image::open("asset/image.png").expect("File Not found !");
    for pixel in img.pixels() {
        pixel.
    }*/

    let mut image : image::RgbaImage = ImageBuffer::new(width, height);
    for y in 0..height {
        for x in 0..width {
            let ray = scene.generate_ray(x, y, width, height);
            //println!("Ray at ({}, {}) is o({}, {}, {}), d({}, {}, {})", x, y, ray.origin.x, ray.origin.y, ray.origin.z, ray.direction.x, ray.direction.y, ray.direction.z);

            if scene.intersect(&ray) {
                image.put_pixel(x, y, image::Rgba([255, 0, 0, 255]))
            } else {
                image.put_pixel(x, y, image::Rgba([0, 0, 0, 255]));
            }
        }
    }
    let path = std::path::Path::new("output.png");
    image
        .save_with_format(path, image::ImageFormat::Png)
        .unwrap();
    println!("Rendering finished!");
}
