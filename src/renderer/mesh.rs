use crate::math::{BndBox2f, Point2f};
use crate::objmodel::ObjModel;
use crate::tgaimage::{TGAColor, TGAImage};
use super::{Camera, draw_triangle};
use rand::Rng;

pub fn draw_mesh(model: &ObjModel,
                 camera: &Camera,
                 image: &mut TGAImage) {
    // Compute 2D camera plane coords of triangle vertices
    let mut pnt2s: Vec<Point2f> = Vec::new();
    for tri in model.triangles.as_slice() {
        for v_idx in tri.vertices {
            let v = model.vertices[v_idx as usize - 1];
            pnt2s.push(camera.project_point(&v));
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

    // Draw the triangles
    let mut rng = rand::thread_rng();
    for i in (0..pnt2s.len()).step_by(3) {
        let (p1, p2, p3) = (
            pnt2s[i].into(),
            pnt2s[i + 1].into(),
            pnt2s[i + 2].into());

        let r: u8 = rng.gen();
        let g: u8 = rng.gen();
        let b: u8 = rng.gen();
        let random_color = TGAColor::from_rgb(r, g, b);
        draw_triangle(p1, p2, p3, image, random_color);
    }
}
