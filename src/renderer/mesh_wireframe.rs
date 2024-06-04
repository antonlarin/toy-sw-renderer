use crate::math::{BndBox2f, Point2f};
use crate::objmodel::ObjModel;
use crate::tgaimage::{TGAColor, TGAImage};
use super::{Camera, draw_line};

pub fn draw_mesh_wireframe(model: &ObjModel,
                           camera: &Camera,
                           image: &mut TGAImage,
                           color: TGAColor) {
    // Compute 2D camera plane coords of triangle edges
    let mut pnt2s: Vec<Point2f> = Vec::new();
    for tri in model.triangles.as_slice() {
        let vert1_iter = tri.vertices.into_iter();
        let vert2_iter = tri.vertices.into_iter();
        for (v1, v2) in vert1_iter.zip(vert2_iter.cycle().skip(1)) {
            let p1 = model.vertices[v1 as usize - 1];
            let p2 = model.vertices[v2 as usize - 1];

            let pnt2_1 = camera.project_point(&p1);
            let pnt2_2 = camera.project_point(&p2);
            pnt2s.push(pnt2_1);
            pnt2s.push(pnt2_2);
        }
    }

    let mut bnd_box = BndBox2f::new_empty();
    for pnt2 in pnt2s.as_slice() {
        bnd_box.add_point(*pnt2);
    }
    let range_vec = bnd_box.max - bnd_box.min;
    const MARGIN_FACTOR: f32 = 1.05;
    let range = range_vec.x.max(range_vec.y) * MARGIN_FACTOR;
    let center = bnd_box.center();
    let image_width = image.width as f32;
    let image_height = image.height as f32;
    for mut p in pnt2s.as_mut_slice() {
        p.x = (p.x - center.x) / range * image_width + image_width / 2.0;
        p.y = (p.y - center.y) / range * image_height + image_height / 2.0;
    }

    // Draw the edges
    for i in (0..pnt2s.len()).step_by(2) {
        let (p1, p2) = (pnt2s[i], pnt2s[i + 1]);
        draw_line(
            p1.x as i32,
            p1.y as i32,
            p2.x as i32,
            p2.y as i32,
            image,
            color,
        );
    }
}

