use crate::math::{BndBox2f, Point2f, Point3f, Vec3f};
use crate::objmodel::ObjModel;
use crate::tgaimage::{TGAColor, TGAImage};
use super::{Camera, draw_triangle};

pub fn draw_mesh(model: &ObjModel,
                 camera: &Camera,
                 light_dir: Vec3f,
                 image: &mut TGAImage,
                 color: TGAColor) {
    // Compute 2D camera plane coords of triangle vertices
    let mut pnt2s: Vec<Point2f> = Vec::new();
    let mut intensities: Vec<f32> = Vec::new();
    for tri in model.triangles.as_slice() {
        let mut vs = [Point3f::origin(); 3];
        for (i, v_idx) in tri.vertices.iter().enumerate() {
            vs[i] = model.vertices[*v_idx as usize - 1];
            pnt2s.push(camera.project_point(&vs[i]));
        }

        // Use reversed normal to avoid having to negate dot
        // product for intensity value
        let rev_normal = (vs[2] - vs[0]).cross(vs[1] - vs[0]).normalize();
        intensities.push(rev_normal.dot(light_dir));
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
    for (i, intens) in (0..pnt2s.len()).step_by(3).zip(intensities) {
        if intens <= 0.0 {
            continue
        }

        let (p1, p2, p3) = (
            pnt2s[i].into(),
            pnt2s[i + 1].into(),
            pnt2s[i + 2].into());

        println!("Intensity {:?}", intens);
        let local_color = color.scale(intens);
        println!("Local color {:?}", local_color);
        draw_triangle(p1, p2, p3, image, local_color);
    }
}
