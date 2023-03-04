mod tgaimage;

use tgaimage::{TGAColor, TGAImage, TGAFormat};

fn main() {
    // let white: TGAColor = TGAColor::from_components(255, 255, 255, 255);
    let red: TGAColor = TGAColor::from_components(255, 0, 0, 255);

    let mut image = TGAImage::with_size(100, 100, TGAFormat::RGB);
    image.set(52, 41, red).unwrap();
    image.flip_vertically().unwrap();
    image.write_to_file("output.tga").unwrap();
}
