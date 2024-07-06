use crate::math::{Point2f, Point3f};
use crate::mesh::IndexedTriangleMesh;
use crate::tgaimage::{TGAColor, TGAImage};
use super::{Context, draw_3d_triangle, draw_3d_triangle_textured};

pub fn draw_mesh(mesh: &IndexedTriangleMesh,
                 ctx: &Context,
                 image: &mut TGAImage,
                 color: TGAColor) {
    let mut z_buf = vec![f32::MAX; (image.width * image.height) as usize];
    for tri in &mesh.triangles {
        let mut vs = [Point3f::origin(); 3];
        for (i, v_idx) in tri.vertices.iter().enumerate() {
            vs[i] = mesh.vertices[(*v_idx - 1) as usize];
        }
        draw_3d_triangle(vs[0], vs[1], vs[2], ctx, image, color, &mut z_buf);
    }
}

pub fn draw_mesh_textured(mesh: &IndexedTriangleMesh,
                          ctx: &Context,
                          image: &mut TGAImage,
                          diff_texture: &TGAImage) {
    let mut z_buf = vec![f32::MAX; (image.width * image.height) as usize];
    for tri in &mesh.triangles {
        let mut vs = [Point3f::origin(); 3];
        let mut tcs = [Point2f::origin(); 3];
        for (i, v_idx) in tri.vertices.iter().enumerate() {
            vs[i] = mesh.vertices[(*v_idx - 1) as usize];
        }
        assert!(mesh.texcoords.is_some(), "draw_mesh_textured called for model without texture coords");
        for (i, tc_idx) in tri.texcoords.as_ref().unwrap().iter().enumerate() {
            tcs[i] = mesh.texcoords.as_ref().unwrap()[(*tc_idx - 1) as usize];
        }
        draw_3d_triangle_textured(vs[0], vs[1], vs[2], tcs[0], tcs[1], tcs[2], ctx, image, diff_texture, &mut z_buf);
    }
}
