mod tgaimage;
mod renderer;

use tgaimage::{tga_format, TGAColor, TGAImage};
use renderer::line;

fn main() {
    let white: TGAColor = TGAColor::from_components(255, 255, 255, 255);
    let red: TGAColor = TGAColor::from_components(255, 0, 0, 255);

    let mut image = TGAImage::with_size(100, 100, tga_format::RGB);
    image.set(52, 41, red).unwrap();

    line(13, 20, 80, 40, &mut image, red);
    line(20, 13, 40, 80, &mut image, red);
    line(80, 40, 13, 20, &mut image, white);

    image.flip_vertically().unwrap();
    image.write_to_file("output.tga").unwrap();
}
