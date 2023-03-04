mod tgaimage;

use tgaimage::{TGAColor, TGAImage, RGB};

const WHITE: TGAColor = TGAColor::from_components(255, 255, 255, 255);
const RED: TGAColor = TGAColor::from_components(255, 0, 0, 255);

fn main() {
    let mut image = TGAImage::with_size(100, 100, RGB);
    image.set(52, 41, RED);
    image.flip_vertically();
    image.write_to_file("output.tga");
}
