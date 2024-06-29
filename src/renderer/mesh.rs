use crate::math::{Point2f, Point3f, Vec3f};
use crate::objmodel::ObjModel;
use crate::tgaimage::{TGAColor, TGAImage};
use super::{Camera, draw_3d_triangle, draw_3d_triangle_textured};

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

pub fn draw_mesh_textured(model: &ObjModel,
                          camera: &Camera,
                          light_dir: Vec3f,
                          image: &mut TGAImage,
                          diff_texture: &TGAImage) {
    let mut z_buf = vec![f32::MAX; (image.width * image.height) as usize];
    for tri in &model.triangles {
        let mut vs = [Point3f::origin(); 3];
        let mut tcs = [Point2f::origin(); 3];
        for (i, v_idx) in tri.vertices.iter().enumerate() {
            vs[i] = model.vertices[(*v_idx - 1) as usize];
        }
        assert!(model.texcoords.is_some(), "draw_mesh_textured called for model without texture coords");
        for (i, tc_idx) in tri.texcoords.as_ref().unwrap().iter().enumerate() {
            tcs[i] = model.texcoords.as_ref().unwrap()[(*tc_idx - 1) as usize];
        }
        draw_3d_triangle_textured(vs[0], vs[1], vs[2], tcs[0], tcs[1], tcs[2], camera, light_dir, image, diff_texture, &mut z_buf);
    }
}
