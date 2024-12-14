use crate::data::Color;
use crate::fb::Framebuffer;
use crate::math::Point2i;

#[allow(dead_code)]
pub fn draw_line_naive<F: Framebuffer>(p0: Point2i, p1: Point2i, buf: &mut F, col: Color) {
    let (mut x0, mut y0, mut x1, mut y1) = (p0.x, p0.y, p1.x, p1.y);
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
            buf.set_pixel(y, x, col).unwrap();
        } else {
            buf.set_pixel(x, y, col).unwrap();
        }
    }
}

pub fn draw_line_fast<F: Framebuffer>(p0: Point2i, p1: Point2i, buf: &mut F, col: Color) {
    let (mut x0, mut y0, mut x1, mut y1) = (p0.x, p0.y, p1.x, p1.y);
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
            buf.set_pixel(y, x, col).unwrap();
        } else {
            buf.set_pixel(x, y, col).unwrap();
        }
        acc += dacc;
        if acc > dx {
            y += incy;
            acc -= dx * 2;
        }
    }
}

#[cfg(test)]
mod test {
    use super::{draw_line_naive, draw_line_fast};
    use crate::data::Color;
    use crate::fb::Bitmap;

    fn setup_1_image() -> (Bitmap, Color) {
        (Bitmap::with_size(6, 6), Color::white())
    }

    fn setup_2_images() -> (Bitmap, Bitmap, Color) {
        (Bitmap::with_size(6, 6), Bitmap::with_size(6, 6), Color::white())
    }

    #[test]
    fn draw_positive_sloped_line_is_symmetric() {
        let body = |draw_fn| {
            let (mut img1, mut img2, white) = setup_2_images();

            draw_fn(1, 2, 4, 5, &mut img1, white);
            draw_fn(4, 5, 1, 2, &mut img2, white);

            assert_eq!(img1, img2);
        };

        body(draw_line_naive);
        body(draw_line_fast);
    }

    #[test]
    fn draw_negative_sloped_line_is_symmetric() {
        let body = |draw_fn| {
            let (mut img1, mut img2, white) = setup_2_images();

            draw_fn(2, 5, 4, 1, &mut img1, white);
            draw_fn(4, 1, 2, 5, &mut img2, white);

            assert_eq!(img1, img2);
        };

        body(draw_line_naive);
        body(draw_line_fast);
    }

    #[test]
    fn draw_x_aligned_line() {
        let body = |draw_fn| {
            let (mut img, white) = setup_1_image();
            let black = Color::black();

            draw_fn(0, 3, 5, 3, &mut img, white);

            for y in 0..img.height {
                let expected_white = if y == 3 { white } else { black };
                for x in 0..img.width {
                    assert_eq!(img.get_pixel(x, y).unwrap(), expected_white, "@ ({}, {})", x, y);
                }
            }
        };

        body(draw_line_naive);
        body(draw_line_fast);
    }

    #[test]
    fn draw_y_aligned_line() {
        let body = |draw_fn| {
            let (mut img, white) = setup_1_image();
            let black = Color::black();

            draw_fn(2, 0, 2, 5, &mut img, white);

            for x in 0..img.width {
                let expected_white = if x == 2 { white } else { black };
                for y in 0..img.height {
                    assert_eq!(img.get_pixel(x, y).unwrap(), expected_white, "@ ({}, {})", x, y);
                }
            }
        };

        body(draw_line_naive);
        body(draw_line_fast);
    }
}
