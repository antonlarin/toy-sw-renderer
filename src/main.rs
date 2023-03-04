mod tgaimage;

use tgaimage::TGAColor;

fn main() {
    let color = TGAColor::new();
    println!("{:?}", color);
    println!("Hello, world!");
}
