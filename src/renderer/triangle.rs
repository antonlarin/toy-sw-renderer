use crate::math::Point2i;
use crate::tgaimage::{TGAColor, TGAImage};

pub fn draw_triangle(v1: Point2i, v2: Point2i, v3: Point2i, image: &mut TGAImage, color: TGAColor) {
    if v1.y == v2.y && v2.y == v3.y {
        let start = v1.x.min(v2.x.min(v3.x));
        let end = v1.x.max(v2.x.max(v3.x));
        for x in start..=end {
            image.set(x, v1.y, color).unwrap();
        }
        return
    }

    let (mut p1, mut p2, mut p3) = (v1, v2, v3);
    if p2.y < p1.y { std::mem::swap(&mut p1, &mut p2); }
    if p3.y < p2.y { std::mem::swap(&mut p2, &mut p3); }
    if p2.y < p1.y { std::mem::swap(&mut p1, &mut p2); }

    let upside_down = p1.y == p2.y;
    let (mut apex, mut left, mut right) = if upside_down {
        (p3, p1, p2)
    } else {
        (p1, p2, p3)
    };
    if left.x > right.x {
        std::mem::swap(&mut left, &mut right);
    }

    // draw first half of the triangle: 'apex' to 'closest to apex by y from left, right'
    let apex_range = if upside_down { left.y.max(right.y)..=apex.y } else { apex.y..=left.y.min(right.y) };
    for y in apex_range {
        let left_x = apex.x + (left.x - apex.x) * (y - apex.y) / (left.y - apex.y);
        let right_x = apex.x + (right.x - apex.x) * (y - apex.y) / (right.y - apex.y);

        for x in left_x..=right_x {
            image.set(x, y, color).unwrap();
        }
    }

    // second half of triangles does not exist here
    if left.y == right.y {
        return
    }

    // draw second half of the triangle: 'closest to apex' to 'furthest from apex'
    let bottom_range = left.y.min(right.y)..=left.y.max(right.y);
    if left.y < right.y {
        std::mem::swap(&mut right, &mut apex);
    } else /* left.y > right.y */ {
        std::mem::swap(&mut left, &mut apex);
    }
    for y in bottom_range {
        let left_x = apex.x + (left.x - apex.x) * (y - apex.y) / (left.y - apex.y);
        let right_x = apex.x + (right.x - apex.x) * (y - apex.y) / (right.y - apex.y);

        for x in left_x..=right_x {
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

