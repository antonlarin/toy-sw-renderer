extern crate swrender;

use swrender::tgaimage::{tga_format, TGAColor, TGAImage};

fn main() {
    let white: TGAColor = TGAColor::from_components(255, 255, 255, 255);

    let mut image = TGAImage::with_size(100, 100, tga_format::RGB);

    // read obj
    // send triangles to wireframe rendering facility
    image.set(15, 20, white).unwrap();

    image.flip_vertically().unwrap();
    image.write_to_file("output.tga").unwrap();
}

