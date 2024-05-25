use crate::tgaimage::{TGAColor, TGAImage};

#[allow(dead_code)]
fn line_naive(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, image: &mut TGAImage, color: TGAColor) {
    let mut steep = false;
    if (x1 - x0).abs() < (y1 - y0).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }

    if x1 < x0 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    for x in x0..=x1 {
        let y = y0 + (y1 - y0) * (x - x0) / (x1 - x0);
        if steep {
            image.set(y, x, color).unwrap();
        } else {
            image.set(x, y, color).unwrap();
        }
    }
}

fn line_faster(mut x0: i32, mut y0: i32, mut x1: i32, mut y1: i32, image: &mut TGAImage, color: TGAColor) {
    let mut steep = false;
    if (x1 - x0).abs() < (y1 - y0).abs() {
        std::mem::swap(&mut x0, &mut y0);
        std::mem::swap(&mut x1, &mut y1);
        steep = true;
    }

    if x1 < x0 {
        std::mem::swap(&mut x0, &mut x1);
        std::mem::swap(&mut y0, &mut y1);
    }

    let dx = x1 - x0;
    let dy = y1 - y0;
    let incy = if dy > 0 { 1 } else { -1 };
    let mut y = y0;
    let dacc = dy * 2;
    let mut acc = 0;
    for x in x0..=x1 {
        if steep {
            image.set(y, x, color).unwrap();
        } else {
            image.set(x, y, color).unwrap();
        }
        acc += dacc;
        if acc > dx {
            y += incy;
            acc -= dx * 2;
        }
    }
}

pub fn draw_line (x0: i32, y0: i32, x1: i32, y1: i32, image: &mut TGAImage, color: TGAColor) {
    // line_naive(x0, y0, x1, y1, image, color);
    line_faster(x0, y0, x1, y1, image, color);
}
