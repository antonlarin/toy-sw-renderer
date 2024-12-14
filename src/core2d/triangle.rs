use crate::data::Color;
use crate::fb::Framebuffer;
use crate::math::{BndBox2i, Point2i, Vec3f, Vec3i};

#[allow(dead_code)]
pub fn draw_triangle_sweep<F: Framebuffer>(v1: Point2i, v2: Point2i, v3: Point2i, buf: &mut F, col: Color) {
    // handle degenerate triangle first
    if v1.y == v2.y && v2.y == v3.y {
        let xl = v1.x.min(v2.x.min(v3.x));
        let xr = v1.x.max(v2.x.max(v3.x));
        for x in xl..=xr {
            buf.set_pixel(x, v1.y, col).unwrap();
        }
        return;
    }

    let mut vs = [v1, v2, v3];
    vs.sort_by_key(|v| v.y);
    let (p1, p2, p3) = (vs[0], vs[1], vs[2]);

    let total_height = p3.y - p1.y;
    for i in 0..=total_height {
        let second_half = i > p2.y - p1.y || p1.y == p2.y;
        let segment_height = if second_half { p3.y - p2.y } else { p2.y - p1.y };
        let local_i = if second_half { i - p2.y + p1.y } else { i };

        let mut xl = p1.x + i * (p3.x - p1.x) / total_height;
        let mut xr = if !second_half {
            p1.x + i * (p2.x - p1.x) / segment_height
        } else {
            p2.x + local_i * (p3.x - p2.x) / segment_height
        };

        if xl > xr { std::mem::swap(&mut xl, &mut xr); }
        for x in xl..=xr {
            buf.set_pixel(x, p1.y + i, col).unwrap();
        }
    }
}

pub fn draw_triangle_parallel<F: Framebuffer>(v1: Point2i, v2: Point2i, v3: Point2i, buf: &mut F, col: Color) {
    if v1.y == v2.y && v2.y == v3.y {
        let xl = v1.x.min(v2.x.min(v3.x));
        let xr = v1.x.max(v2.x.max(v3.x));
        for x in xl..=xr { buf.set_pixel(x, v1.y, col).unwrap(); }
        return;
    } else if v1.x == v2.x && v2.x == v3.x {
        let yt = v1.y.min(v2.y.min(v3.y));
        let yb = v1.y.max(v2.y.max(v3.y));
        for y in yt..=yb { buf.set_pixel(v1.x, y, col).unwrap(); }
        return;
    }


    let mut bbox = BndBox2i::new_empty();
    bbox.add_point(v1);
    bbox.add_point(v2);
    bbox.add_point(v3);

    let is_inside = |p: Point2i| {
        // solve linear eqn: p = 1 * v1 + u * v2 + v * v3;
        let aux1 = Vec3i { x: v2.x - v1.x, y: v3.x - v1.x, z: v1.x - p.x };
        let aux2 = Vec3i { x: v2.y - v1.y, y: v3.y - v1.y, z: v1.y - p.y };
        let solution = aux1.cross(aux2);
        if solution.z == 0 {
            return false
        }

        let barycentric = Vec3f {
            x: 1.0f32 - (solution.x + solution.y) as f32 / solution.z as f32,
            y: solution.x as f32 / solution.z as f32,
            z: solution.y as f32 / solution.z as f32,
        };
        if barycentric.x < 0.0 || barycentric.y < 0.0 || barycentric.z < 0.0 {
            return false
        } else {
            return true
        }
    };

    // TODO: try to parallelize with rayon
    for x in bbox.min.x..=bbox.max.x {
        for y in bbox.min.y..=bbox.max.y {
            if is_inside(Point2i { x, y }) {
                buf.set_pixel(x, y, col).unwrap();
            }
        }
    }
}

//pub fn draw_triangle(v1: Point2i, v2: Point2i, v3: Point2i, image: &mut TGAImage, color: TGAColor) {
//    draw_triangle_parallel(v1, v2, v3, image, color);
//}

#[cfg(test)]
mod test {
    use super::{draw_triangle_sweep, draw_triangle_parallel};
    use crate::data::Color;
    use crate::fb::Bitmap;
    use crate::math::Point2i;

    fn setup_1_image() -> (Bitmap, Color) {
        (Bitmap::with_size(6, 6), Color::white())
    }

    fn setup_2_images() -> (Bitmap, Bitmap, Color) {
        (Bitmap::with_size(6, 6), Bitmap::with_size(6, 6), Color::white())
    }

    #[test]
    fn different_vertex_order() {
        let body = |draw_fn| {
            let (mut img1, mut img2, white) = setup_2_images();

            let v1 = Point2i { x: 3, y: 0 };
            let v2 = Point2i { x: 5, y: 5 };
            let v3 = Point2i { x: 1, y: 3 };
            draw_fn(v1, v2, v3, &mut img1, white);
            draw_fn(v1, v3, v2, &mut img2, white);

            assert_eq!(img1, img2);
        };

        body(draw_triangle_sweep);
        body(draw_triangle_parallel);
    }

    #[test]
    fn degenerate_x_triangle() {
        let body = |draw_fn| {
            let (mut img, white) = setup_1_image();
            let black = Color::black();

            let v1 = Point2i { x: 0, y: 3 };
            let v2 = Point2i { x: 2, y: 3 };
            let v3 = Point2i { x: 5, y: 3 };

            draw_fn(v1, v2, v3, &mut img, white);
            // TODO: implement Bitmap writing to TGA
            //img.write_to_file("assets/test_degen_x.tga").unwrap();

            for y in 0..img.height {
                let expected_white = if y == 3 { white } else { black };
                for x in 0..img.width {
                    assert_eq!(img.get_pixel(x, y).unwrap(), expected_white, "@ ({}, {})", x, y);
                }
            }
        };

        body(draw_triangle_sweep);
        body(draw_triangle_parallel);
    }

    #[test]
    fn degenerate_y_triangle() {
        let body = |draw_fn| {
            let (mut img, white) = setup_1_image();
            let black = Color::black();

            let v1 = Point2i { x: 2, y: 0 };
            let v2 = Point2i { x: 2, y: 5 };
            let v3 = Point2i { x: 2, y: 2 };

            draw_fn(v1, v2, v3, &mut img, white);
            // TODO: implement Bitmap writing to TGA
            //img.write_to_file("assets/test_degen_y.tga").unwrap();

            for x in 0..img.width {
                let expected_white = if x == 2 { white } else { black };
                for y in 0..img.height {
                    assert_eq!(img.get_pixel(x, y).unwrap(), expected_white, "@ ({}, {})", x, y);
                }
            }
        };

        body(draw_triangle_sweep);
        body(draw_triangle_parallel);
    }

    #[test]
    fn corner_right_triangles() {
        let body = |draw_fn| {
            let white = Color::white();
            let black = Color::black();

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
                let mut img = Bitmap::with_size(6, 6);
                draw_fn(vs[i], vs[(i + 1) % 4], vs[(i + 2) % 4], &mut img, white);
                // TODO: implement Bitmap writing to TGA
                //img.write_to_file(format!("assets/test_corner_tri_{}.tga", i).as_str()).unwrap();

                for x in 0..img.width {
                    for y in 0..img.height {
                        let expected_white = if preds[i](x, y) { white } else { black };
                        assert_eq!(img.get_pixel(x, y).unwrap(), expected_white, "@ ({}, {}) triangle at corner {}", x, y, i);
                    }
                }
            }
        };

        body(draw_triangle_sweep);
        body(draw_triangle_parallel);
    }
}
