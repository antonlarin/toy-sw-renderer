extern crate swrender;

use swrender::renderer::draw_triangle;
use swrender::math::Point2i;
use swrender::tgaimage::{tga_format, TGAColor, TGAImage};

fn main() {
    let white: TGAColor = TGAColor::from_rgb(255, 255, 255);
    let red: TGAColor = TGAColor::from_rgb(255, 0, 0);
    let green: TGAColor = TGAColor::from_rgb(0, 255, 0);

    let mut image = TGAImage::with_size(200, 200, tga_format::RGB);

    let v1_1 = Point2i { x: 20, y: 20 };
    let v1_2 = Point2i { x: 30, y: 110 };
    let v1_3 = Point2i { x: 100, y: 160 };
    draw_triangle(v1_1, v1_2, v1_3, &mut image, green);

    let v2_1 = Point2i { x: 80, y: 30 };
    let v2_2 = Point2i { x: 100, y: 60 };
    let v2_3 = Point2i { x: 120, y: 30 };
    draw_triangle(v2_1, v2_2, v2_3, &mut image, white);

    let v3_1 = Point2i { x: 40, y: 100 };
    let v3_2 = Point2i { x: 180, y: 150 };
    let v3_3 = Point2i { x: 160, y: 45 };
    draw_triangle(v3_1, v3_2, v3_3, &mut image, red);

    image.flip_vertically().unwrap();
    image.write_to_file("assets/threetriangles-tmp.tga").unwrap();
}

