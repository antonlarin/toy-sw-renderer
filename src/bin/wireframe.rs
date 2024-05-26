extern crate swrender;

use swrender::tgaimage::{tga_format, TGAColor, TGAImage};
use swrender::objmodel::ObjModel;
use swrender::math::{Point2, Point3, Vec3};
use swrender::draw_line;

struct Camera {
    dir: Vec3,
    up: Vec3,
}

impl Camera {
    fn new(dir: Vec3, up: Vec3) -> Camera {
        let dir = dir.normalize();
        if let Some(up) = up.normalize().orthogonalize(&dir) {
            Camera { dir, up }
        } else {
            panic!("Wrong params for camera given");
        }
    }
}

/// Projects a 3D point `pnt` onto a plane defined by isometric camera `cam`.
/// The plane's origin is projection of 3D origin onto it.
fn project(cam: &Camera, pnt: &Point3) -> Point2 {
    let mut rad_vec: Vec3 = (*pnt).into();
    rad_vec = rad_vec - rad_vec.dot(&cam.dir) * rad_vec;
    // TODO: compute the x and y from camera 'up' direction

    Point2 { x: 0.0, y: 0.0 }
}

fn main() {
    let white: TGAColor = TGAColor::from_components(255, 255, 255, 255);

    let mut image = TGAImage::with_size(100, 100, tga_format::RGB);
    let model = ObjModel::from_file("assets/african_head.obj").unwrap();
    let camera = Camera::new(Vec3 { x: -1.0, y: -1.0, z: -1.0 },
                             Vec3 { x: 0.0, y: 0.0, z: 1.0 });

    // Compute 2D camera plane coords of triangle edges
    let mut pnt2s: Vec<Point2> = Vec::new();
    for tri in model.triangles {
        let vert1_iter = tri.vertices.into_iter();
        let vert2_iter = tri.vertices.into_iter();
        for (v1, v2) in vert1_iter.zip(vert2_iter.cycle().skip(1)) {
            let p1 = model.vertices[v1 as usize];
            let p2 = model.vertices[v2 as usize];

            pnt2s.push(project(&camera, &p1));
            pnt2s.push(project(&camera, &p2));
        }
    }

    // TODO: compute 2D bbox for points and set up mapping of camera
    // plane coords into screen space coords

    // Draw the edges
    for i in (0..pnt2s.len()).step_by(2) {
        let (p1, p2) = (pnt2s[i], pnt2s[i + 1]);
        draw_line(p1.x as i32, p1.y as i32,
                  p2.x as i32, p2.y as i32,
                  &mut image, white);
    }

    image.flip_vertically().unwrap();
    image.write_to_file("output.tga").unwrap();
}

