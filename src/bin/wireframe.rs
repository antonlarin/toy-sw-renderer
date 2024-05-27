extern crate swrender;

use swrender::draw_line;
use swrender::math::{BndBox2, Point2, Point3, Vec3};
use swrender::objmodel::ObjModel;
use swrender::tgaimage::{tga_format, TGAColor, TGAImage};

struct Camera {
    dir: Vec3,
    up: Vec3,
}

impl Camera {
    fn new(dir: Vec3, up: Vec3) -> Camera {
        let dir = dir.normalize();
        if let Some(up) = up.orthogonalize(dir) {
            Camera { dir, up: up.normalize() }
        } else {
            panic!("Wrong params for camera given");
        }
    }
}

/// Projects a 3D point `pnt` onto a plane defined by isometric camera `cam`.
/// The plane's origin is projection of 3D origin onto it.
fn project(cam: &Camera, pnt: &Point3) -> Point2 {
    let rad_vec: Vec3 = (*pnt).into();
    let vec_in_plane = rad_vec - rad_vec.dot(cam.dir) * rad_vec;
    let y = vec_in_plane.dot(cam.up);
    let x = (vec_in_plane - y * cam.up).norm();
    Point2 { x, y }
}

fn main() {
    let white: TGAColor = TGAColor::from_components(255, 255, 255, 255);

    const IMAGE_SIZE: i32 = 512;
    let mut image = TGAImage::with_size(IMAGE_SIZE, IMAGE_SIZE, tga_format::RGB);
    let model = ObjModel::from_file("assets/african_head.obj").unwrap();
    let camera = Camera::new(
        Vec3 {
            x: -1.0,
            y: -1.0,
            z: -1.0,
        },
        Vec3 {
            x: 0.0,
            y: 0.0,
            z: 1.0,
        },
    );

    // Compute 2D camera plane coords of triangle edges
    let mut pnt2s: Vec<Point2> = Vec::new();
    for tri in model.triangles {
        let vert1_iter = tri.vertices.into_iter();
        let vert2_iter = tri.vertices.into_iter();
        for (v1, v2) in vert1_iter.zip(vert2_iter.cycle().skip(1)) {
            let p1 = model.vertices[v1 as usize - 1];
            let p2 = model.vertices[v2 as usize - 1];

            pnt2s.push(project(&camera, &p1));
            pnt2s.push(project(&camera, &p2));
        }
    }

    let mut bnd_box = BndBox2::new_empty();
    for pnt2 in pnt2s.as_slice() {
        bnd_box.add_point(*pnt2);
    }
    let range_vec = bnd_box.max - bnd_box.min;
    let range = range_vec.x.max(range_vec.y);
    let center = bnd_box.center();
    for mut p in pnt2s.as_mut_slice() {
        p.x = (p.x - center.x) / range * IMAGE_SIZE as f32 + (IMAGE_SIZE / 2) as f32;
        p.y = (p.y - center.y) / range * IMAGE_SIZE as f32 + (IMAGE_SIZE / 2) as f32;
    }

    // Draw the edges
    for i in (0..pnt2s.len()).step_by(2) {
        let (p1, p2) = (pnt2s[i], pnt2s[i + 1]);
        draw_line(
            p1.x as i32,
            p1.y as i32,
            p2.x as i32,
            p2.y as i32,
            &mut image,
            white,
        );
    }

    image.flip_vertically().unwrap();
    image.write_to_file("assets/output.tga").unwrap();
}
