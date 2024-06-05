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

#[cfg(test)]
mod test {
    use crate::tgaimage::{tga_format, TGAImage, TGAColor};
    use super::draw_line;

    fn setup_1_image() -> (TGAImage, TGAColor) {
        (TGAImage::with_size(6, 6, tga_format::RGB),
         TGAColor::from_rgb(255, 255, 255))
    }

    fn setup_2_images() -> (TGAImage, TGAImage, TGAColor) {
        (TGAImage::with_size(6, 6, tga_format::RGB),
         TGAImage::with_size(6, 6, tga_format::RGB),
         TGAColor::from_rgb(255, 255, 255))
    }

    #[test]
    fn draw_positive_sloped_line_is_symmetric() {
        let (mut img1, mut img2, col) = setup_2_images();

        draw_line(1, 2, 4, 5, &mut img1, col);
        draw_line(4, 5, 1, 2, &mut img2, col);

        assert_eq!(img1, img2);
    }

    #[test]
    fn draw_negative_sloped_line_is_symmetric() {
        let (mut img1, mut img2, col) = setup_2_images();

        draw_line(2, 5, 4, 1, &mut img1, col);
        draw_line(4, 1, 2, 5, &mut img2, col);

        assert_eq!(img1, img2);
    }

    #[test]
    fn draw_x_aligned_line() {
        let (mut img, col) = setup_1_image();
        let black = TGAColor::from_rgb(0, 0, 0);

        draw_line(0, 3, 5, 3, &mut img, col);

        for y in 0..img.height {
            let expected_col = if y == 3 { col } else { black };
            for x in 0..img.width {
                assert_eq!(img.get(x, y).unwrap(), expected_col, "@ ({}, {})", x, y);
            }
        }
    }

    #[test]
    fn draw_y_aligned_line() {
        let (mut img, col) = setup_1_image();
        let black = TGAColor::from_rgb(0, 0, 0);

        draw_line(2, 0, 2, 5, &mut img, col);

        for x in 0..img.width {
            let expected_col = if x == 2 { col } else { black };
            for y in 0..img.height {
                assert_eq!(img.get(x, y).unwrap(), expected_col, "@ ({}, {})", x, y);
            }
        }
    }
}
