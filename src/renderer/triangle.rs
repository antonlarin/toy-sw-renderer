use crate::math::Point2i;
use crate::tgaimage::{TGAColor, TGAImage};

pub fn draw_triangle(v1: Point2i, v2: Point2i, v3: Point2i, image: &mut TGAImage, color: TGAColor) {
    let (mut p1, mut p2, mut p3) = (v1, v2, v3);
    if p2.y < p1.y { std::mem::swap(&mut p1, &mut p2); }
    if p3.y < p2.y { std::mem::swap(&mut p2, &mut p3); }
    if p2.y < p1.y { std::mem::swap(&mut p1, &mut p2); }

    // TODO: fix x-degen triangle
    // TODO: fix corner tests
    let mut lmul = if p2.y == p1.y { (1, 1) } else { (p2.x - p1.x, p2.y - p1.y) };
    let mut rmul = if p3.y == p1.y { (1, 1) } else { (p3.x - p1.x, p3.y - p1.y) };
    {
        let ltest = p1.x + 1 * lmul.0 / lmul.1;
        let rtest = p1.x + 1 * rmul.0 / rmul.1;
        if ltest > rtest {
            std::mem::swap(&mut lmul, &mut rmul);
        }
    }
    for y in p1.y..p2.y {
        let xl = p1.x + (y - p1.y) * lmul.0 / lmul.1;
        let xr = p1.x + (y - p1.y) * rmul.0 / rmul.1;
        for x in xl..=xr {
            image.set(x, y, color).unwrap();
        }
    }

    lmul = if p1.y == p3.y { (1, 1) } else { (p1.x - p3.x, p1.y - p3.y) };
    rmul = if p2.y == p3.y { (1, 1) } else { (p2.x - p3.x, p2.y - p3.y) };
    {
        let ltest = p3.x - 1 * lmul.0 / lmul.1;
        let rtest = p3.x - 1 * rmul.0 / rmul.1;
        if ltest > rtest {
            std::mem::swap(&mut lmul, &mut rmul);
        }
    }
    for y in p2.y..=p3.y {
        let xl = p3.x + (y - p3.y) * lmul.0 / lmul.1;
        let xr = p3.x + (y - p3.y) * rmul.0 / rmul.1;
        for x in xl..=xr {
            image.set(x, y, color).unwrap();
        }
    }
}

#[cfg(test)]
mod test {
    use super::{draw_triangle, TGAImage, TGAColor, Point2i};
    use crate::tgaimage::tga_format;

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
    fn different_vertex_order() {
        let (mut img1, mut img2, col) = setup_2_images();

        let v1 = Point2i { x: 3, y: 0 };
        let v2 = Point2i { x: 5, y: 5 };
        let v3 = Point2i { x: 1, y: 3 };
        draw_triangle(v1, v2, v3, &mut img1, col);
        draw_triangle(v1, v3, v2, &mut img2, col);

        assert_eq!(img1, img2);
    }

    #[test]
    fn degenerate_x_triangle() {
        let mut img = TGAImage::with_size(32, 32, tga_format::RGB);
        let col = TGAColor::from_rgb(255, 255, 255);
        // let (mut img, col) = setup_1_image();
        let black = TGAColor::from_rgb(0, 0, 0);

        let v1 = Point2i { x: 0, y: 16 };
        let v2 = Point2i { x: 14, y: 16 };
        let v3 = Point2i { x: 31, y: 16 };

        draw_triangle(v1, v2, v3, &mut img, col);
        img.write_to_file("assets/test_degen_x.tga").unwrap();

        for y in 0..img.height {
            let expected_col = if y == 3 { col } else { black };
            for x in 0..img.width {
                assert_eq!(img.get(x, y).unwrap(), expected_col, "@ ({}, {})", x, y);
            }
        }
    }

    #[test]
    fn degenerate_y_triangle() {
        let (mut img, col) = setup_1_image();
        let black = TGAColor::from_rgb(0, 0, 0);

        let v1 = Point2i { x: 2, y: 0 };
        let v2 = Point2i { x: 2, y: 5 };
        let v3 = Point2i { x: 2, y: 2 };

        draw_triangle(v1, v2, v3, &mut img, col);
        img.write_to_file("assets/test_degen_y.tga").unwrap();

        for x in 0..img.width {
            let expected_col = if x == 2 { col } else { black };
            for y in 0..img.height {
                assert_eq!(img.get(x, y).unwrap(), expected_col, "@ ({}, {})", x, y);
            }
        }
    }

    #[test]
    fn corner_right_triangles() {
        let (mut img, col) = setup_1_image();
        let black = TGAColor::from_rgb(0, 0, 0);

        let vs = [
            Point2i { x: 0, y: 0 },
            Point2i { x: 5, y: 0 },
            Point2i { x: 5, y: 5 },
            Point2i { x: 0, y: 5 },
        ];
        let preds = [
            |x, y| x >= y,
            |x, y| x + y >= 5,
            |x, y| y >= x,
            |x, y| x + y <= 5,
        ];

        for i in 0..4 {
            draw_triangle(vs[i], vs[(i + 1) % 4], vs[(i + 2) % 4], &mut img, col);
            img.write_to_file(format!("assets/test_corner_tri_{}.tga", i).as_str()).unwrap();

            for x in 0..img.width {
                for y in 0..img.height {
                    let expected_col = if preds[i](x, y) { col } else { black };
                    assert_eq!(img.get(x, y).unwrap(), expected_col, "@ ({}, {}) triangle at corner {}", x, y, i);
                }
            }

            img.clear();
        }
    }
}

