extern crate swrender;

use swrender::renderer::{Camera, draw_mesh_wireframe};
use swrender::math::{Point3f, Vec3};
use swrender::obj;
use swrender::tgaimage::{tga_format, TGAColor, TGAImage};

#[allow(unused_variables)]
fn main() {
    let white: TGAColor = TGAColor::from_rgb(255, 255, 255);

    const IMAGE_SIZE: i32 = 960;
    let mut image = TGAImage::with_size(IMAGE_SIZE, IMAGE_SIZE, tga_format::RGB);
    let model = obj::load_obj_file("assets/african_head.obj").unwrap();

    let camera_xp_yp_zp = Camera::new(
        Point3f { x: 1.0, y: 0.3, z: 1.0 },
        Vec3 { x: -1.0, y: -0.3, z: -1.0 },
        Vec3 { x: 0.0, y: 1.0, z: 0.0 },
        0.9
    );

    draw_mesh_wireframe(&model, &camera_xp_yp_zp, &mut image, white);

    image.flip_vertically().unwrap();
    image.write_to_file("assets/wireframe.tga").unwrap();
}
