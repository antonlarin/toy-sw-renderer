use crate::math::{Point3f, Vec3f};
use crate::objmodel::ObjModel;
use crate::tgaimage::{TGAColor, TGAImage};
use super::{Camera, draw_3d_triangle};

pub fn draw_mesh(model: &ObjModel,
                 camera: &Camera,
                 light_dir: Vec3f,
                 image: &mut TGAImage,
                 color: TGAColor) {
    let mut z_buf = vec![f32::MAX; (image.width * image.height) as usize];
    let yellow = TGAColor::from_rgb(255, 255, 0);
    let blue = TGAColor::from_rgb(0, 0, 255);
    let green = TGAColor::from_rgb(0, 255, 0);
    for (j, tri) in model.triangles.iter().enumerate() {
        let mut vs = [Point3f::origin(); 3];
        for (i, v_idx) in tri.vertices.iter().enumerate() {
            vs[i] = model.vertices[(*v_idx - 1) as usize];
        }
        draw_3d_triangle(vs[0], vs[1], vs[2], camera, light_dir, image, color, &mut z_buf);
    }
}
