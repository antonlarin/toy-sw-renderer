extern crate swrender;

use swrender::tgaimage::{tga_format, TGAColor, TGAImage};
use swrender::objmodel::{ObjModel, Triangle};
use swrender::draw_line;

fn main() {
    let white: TGAColor = TGAColor::from_components(255, 255, 255, 255);

    let mut image = TGAImage::with_size(100, 100, tga_format::RGB);

    let model = ObjModel::from_file("assets/african_head.obj").unwrap();

    // TODO: Generalize with projection matrix and model bounding box
    for tri in model.triangles {
        let vert1_iter = tri.vertices.into_iter();
        let vert2_iter = tri.vertices.into_iter();
        for (v1, v2) in vert1_iter.zip(vert2_iter.cycle().take(1)) {
            let p1 = model.vertices[v1 as usize];
            let p2 = model.vertices[v2 as usize];

            // try to do isometric projection
            let x0 = 0;
            let y0 = 0;
            let x1 = 0;
            let y1 = 0;

            draw_line(x0, y0, x1, y1, &mut image, white);
        }
    }

    image.flip_vertically().unwrap();
    image.write_to_file("output.tga").unwrap();
}

