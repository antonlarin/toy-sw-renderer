extern crate swrender;

use swrender::core2d::draw_line;
use swrender::data::Color;
use swrender::fb::Bitmap;
use swrender::util::write_tga;

fn main() {
    let white = Color::white();
    let red = Color::red();

    let mut image = Bitmap::with_size(100, 100);
    image.set_pixel(52, 41, red).unwrap();

    draw_line(13, 20, 80, 40, &mut image, red);
    draw_line(20, 13, 40, 80, &mut image, red);
    draw_line(80, 40, 13, 20, &mut image, white);

    image.flip_vertically().unwrap();
    write_tga(image, "assets/threelines.tga").unwrap();
}
