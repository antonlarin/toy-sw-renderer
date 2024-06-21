extern crate swrender;

use swrender::renderer::{Camera, draw_mesh};
use swrender::math::{Point3f, Vec3f};
use swrender::objmodel::ObjModel;
use swrender::tgaimage::{tga_format, TGAColor, TGAImage};

#[allow(unused_variables)]
fn main() {
    const IMAGE_SIZE: i32 = 512;
    let mut image = TGAImage::with_size(IMAGE_SIZE, IMAGE_SIZE, tga_format::RGB);
    let model = ObjModel::from_file("assets/african_head.obj").unwrap();
    let color = TGAColor::from_rgb(93, 76, 69);
    let light_dir = Vec3f { x: -1.0, y: 0.0, z: -1.0 }.normalize();

    let camera_xp_yp_zp = Camera::new(
        Point3f { x: 1.0, y: 0.3, z: 1.0 },
        Vec3f { x: -1.0, y: -0.3, z: -1.0 },
        Vec3f { x: 0.0, y: 1.0, z: 0.0 },
        210.0
    );

    draw_mesh(&model, &camera_xp_yp_zp, light_dir, &mut image, color);

    image.flip_vertically().unwrap();
    image.write_to_file("assets/mesh_head.tga").unwrap();
}

