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
    for tri in &model.triangles {
        let mut vs = [Point3f::origin(); 3];
        for (i, v_idx) in tri.vertices.iter().enumerate() {
            vs[i] = model.vertices[(*v_idx - 1) as usize];
        }
        draw_3d_triangle(vs[0], vs[1], vs[2], camera, light_dir, image, color, &mut z_buf);
    }
}
