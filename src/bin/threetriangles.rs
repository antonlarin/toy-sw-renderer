extern crate swrender;

use swrender::renderer::draw_triangle;
use swrender::math::Point2;
use swrender::tgaimage::{tga_format, TGAColor, TGAImage};

fn main() {
    let white: TGAColor = TGAColor::from_components(255, 255, 255, 255);
    let red: TGAColor = TGAColor::from_components(255, 0, 0, 255);
    let green: TGAColor = TGAColor::from_components(0, 255, 0, 255);

    let mut image = TGAImage::with_size(200, 200, tga_format::RGB);

    let v1_1 = Point2 { x: 20.0, y: 20.0 };
    let v1_2 = Point2 { x: 30.0, y: 110.0 };
    let v1_3 = Point2 { x: 100.0, y: 160.0 };
    draw_triangle(v1_1, v1_2, v1_3, &mut image, green);

    let v2_1 = Point2 { x: 80.0, y: 30.0 };
    let v2_2 = Point2 { x: 100.0, y: 60.0 };
    let v2_3 = Point2 { x: 120.0, y: 30.0 };
    draw_triangle(v2_1, v2_2, v2_3, &mut image, white);

    let v3_1 = Point2 { x: 40.0, y: 100.0 };
    let v3_2 = Point2 { x: 180.0, y: 150.0 };
    let v3_3 = Point2 { x: 160.0, y: 45.0 };
    draw_triangle(v3_1, v3_2, v3_3, &mut image, red);

    image.flip_vertically().unwrap();
    image.write_to_file("output.tga").unwrap();
}

