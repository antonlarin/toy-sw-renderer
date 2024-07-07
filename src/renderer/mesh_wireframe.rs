use crate::math::{BndBox2f, Point2f};
use crate::mesh::IndexedTriangleMesh;
use crate::tgaimage::{TGAColor, TGAImage};
use super::{Camera, draw_line};

pub fn draw_mesh_wireframe(model: &IndexedTriangleMesh,
                           camera: &Camera,
                           image: &mut TGAImage,
                           color: TGAColor) {
    let iw = (image.width - 1) as f32;
    let ih = (image.height - 1) as f32;
    let img_half_dims = Point2f { x: 0.5 * iw, y: 0.5 * ih };
    let mut screen_bbox = BndBox2f::new_empty();
    screen_bbox.add_point(Point2f { x: -iw * 0.5, y: -ih * 0.5 });
    screen_bbox.add_point(Point2f { x: iw * 0.5, y: ih * 0.5 });

    // Compute 2D camera plane coords of triangle edges
    let mut pnt2s: Vec<Point2f> = Vec::new();
    for tri in model.triangles.as_slice() {
        let vert1_iter = tri.vertices.into_iter();
        let vert2_iter = tri.vertices.into_iter();
        for (v1, v2) in vert1_iter.zip(vert2_iter.cycle().skip(1)) {
            let p1 = model.vertices[v1 as usize - 1];
            let p2 = model.vertices[v2 as usize - 1];

            let pnt2_1 = camera.project(&p1) * img_half_dims;
            let pnt2_2 = camera.project(&p2) * img_half_dims;
            pnt2s.push(screen_bbox.clamp(pnt2_1));
            pnt2s.push(screen_bbox.clamp(pnt2_2));
        }
    }

    // Draw the edges
    for i in (0..pnt2s.len()).step_by(2) {
        let (p1, p2) = (pnt2s[i], pnt2s[i + 1]);
        draw_line(
            (iw * 0.5 + p1.x) as i32,
            (ih * 0.5 + p1.y) as i32,
            (iw * 0.5 + p2.x) as i32,
            (ih * 0.5 + p2.y) as i32,
            image,
            color,
        );
    }
}

