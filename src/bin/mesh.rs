extern crate swrender;

use swrender::renderer::{Camera, draw_mesh};
use swrender::math::Vec3;
use swrender::objmodel::ObjModel;
use swrender::tgaimage::{tga_format, TGAImage};

#[allow(unused_variables)]
fn main() {
    const IMAGE_SIZE: i32 = 512;
    let mut image = TGAImage::with_size(IMAGE_SIZE, IMAGE_SIZE, tga_format::RGB);
    let model = ObjModel::from_file("assets/african_head.obj").unwrap();

    let camera_xp_yp_zp = Camera::new(
        Vec3 { x: -1.0, y: -0.3, z: -1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 });

    // TODO: add light and flat shading (pass light and color)
    draw_mesh(&model, &camera_xp_yp_zp, &mut image);

    image.flip_vertically().unwrap();
    image.write_to_file("assets/wireframe.tga").unwrap();
}

