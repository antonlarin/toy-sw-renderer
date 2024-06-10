use crate::math::Point2i;
use crate::tgaimage::{TGAColor, TGAImage};

pub fn draw_triangle(v1: Point2i, v2: Point2i, v3: Point2i, image: &mut TGAImage, color: TGAColor) {
    let (mut p1, mut p2, mut p3) = (v1, v2, v3);
    if p2.y < p1.y { std::mem::swap(&mut p1, &mut p2); }
    if p3.y < p2.y { std::mem::swap(&mut p2, &mut p3); }
    if p2.y < p1.y { std::mem::swap(&mut p1, &mut p2); }

    let mut pl = p2;
    let mut pr = p3;
    if pl.x > pr.x { std::mem::swap(&mut pl, &mut pr); } // TODO: wrong for thin triangles
    let lmul = if pl.y == p1.y { 1 } else { (pl.x - p1.x) / (pl.y - p1.y) };
    let rmul = if pr.y == p1.y { 1 } else { (pr.x - p1.x) / (pr.y - p1.y) };
    for y in p1.y..p2.y {
        let xl = p1.x + (y - p1.y) * lmul;
        let xr = p1.x + (y - p1.y) * rmul;
        for x in xl..=xr {
            image.set(x, y, color).unwrap();
        }
    }

    pl = p1;
    pr = p2;
    if pl.x > pr.x { std::mem::swap(&mut pl, &mut pr); } // TODO: wrong for thin triangles
    for y in p2.y..=p3.y {
        let mut xl = p3.x;
        let mut xr = p3.x;
        if pl.y - p3.y != 0 {
            xl += (y - p3.y) * (pl.x - p3.x) / (pl.y - p3.y)
        }
        if pr.y - p3.y != 0 {
            xr += (y - p3.y) * (pr.x - p3.x) / (pr.y - p3.y)
        }

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
        let (mut img, col) = setup_1_image();
        let black = TGAColor::from_rgb(0, 0, 0);

        let v1 = Point2i { x: 0, y: 3 };
        let v2 = Point2i { x: 3, y: 3 };
        let v3 = Point2i { x: 5, y: 3 };

        draw_triangle(v1, v2, v3, &mut img, col);

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

