extern crate swrender;

use swrender::renderer::{Camera, draw_mesh_textured};
use swrender::math::{Point3f, Vec3f};
use swrender::objmodel::ObjModel;
use swrender::tgaimage::{tga_format, TGAImage};

#[allow(unused_variables)]
fn main() {
    const IMAGE_SIZE: i32 = 1024;
    let mut image = TGAImage::with_size(IMAGE_SIZE, IMAGE_SIZE, tga_format::RGB);
    let model = ObjModel::from_file("assets/african_head.obj").unwrap();
    let texture = TGAImage::from_tga_file("assets/african_head_diffuse.tga").unwrap();
    let light_dir = Vec3f { x: -3.0, y: -1.0, z: -3.0 }.normalize();

    // TODO: image size is currently physical,
    // so zoom must change when image size is changed
    // Need zoom to fix a specific screen plane, then
    // the image on that plane can be rendered at various resolutions
    let camera_xp_yp_zp = Camera::new(
        Point3f { x: 1.0, y: 0.2, z: 1.0 },
        Vec3f { x: -1.0, y: -0.3, z: -1.0 },
        Vec3f { x: 0.0, y: 1.0, z: 0.0 },
        400.0
    );

    draw_mesh_textured(&model, &camera_xp_yp_zp, light_dir, &mut image, &texture);

    image.flip_vertically().unwrap();
    image.write_to_file("assets/mesh_head.tga").unwrap();
}

