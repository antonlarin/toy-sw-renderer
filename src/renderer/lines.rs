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
    let dacc = dy.abs() * 2;
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

/*
pub struct LinePixelsIterator {
    x: i32,
    y: i32,
    x1: i32,
    steep: bool,
    dx: i32,
    incy: i32,
    dacc: i32,
    acc: i32,
}

impl LinePixelsIterator {
    fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        let mut x0 = x0;
        let mut x1 = x1;
        let mut y0 = y0;
        let mut y1 = y1;
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
        let dacc = dy.abs() * 2;
        let acc = 0;
        let x = x0;
        let y = y0;

        LinePixelsIterator {
            x, y, x1, steep,
            dx, incy,
            dacc, acc,
        }
    }
}

impl Iterator for LinePixelsIterator {
    type Item = (i32, i32);

    fn next(&mut self) -> Option<Self::Item> {
        if self.x > self.x1 {
            return None;
        }

        let mut res = None;
        if self.steep {
            res = Some((self.y, self.x));
        } else {
            res = Some((self.x, self.y));
        }

        self.x += 1;
        self.acc += self.dacc;
        if self.acc > self.dx {
            self.y += self.incy;
            self.acc -= self.dx * 2;
        }

        res
    }
}

pub fn line_pixels_iter (x0: i32, y0: i32, x1: i32, y1: i32) -> LinePixelsIterator {
    LinePixelsIterator::new(x0, y0, x1, y1)
}
*/

#[cfg(test)]
mod test {
    // TODO: add tests checking that order of vertices in draw_line doesn't matter
    // Types of lines: axis-aligned, different slopes, single-pixel line
}
