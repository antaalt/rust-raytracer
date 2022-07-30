use image::ImageBuffer;

mod geo;
mod scene;
mod primitives;

use geo::Vector3;
use scene::Scene;


fn main() {
    let width : u32 = 800;
    let height : u32 = 600;
    let scene = Scene::new();

    let image : image::RgbaImage = ImageBuffer::from_fn(width, height, |x, y| {
        let ray = scene.generate_ray(x, height - y, width, height);
        let intersection = scene.intersect(ray);
        if intersection.valid() {
            let color = intersection.shade();
            let color32 = color.to_ldr();
            return image::Rgba([color32.r, color32.g, color32.b, color32.a])
        } else {
            // TODO envmap
            return image::Rgba([66, 135, 245, 255])
        }
    });
    
    let path = std::path::Path::new("output.png");
    image
        .save_with_format(path, image::ImageFormat::Png)
        .unwrap();
    println!("Rendering finished!");
}
