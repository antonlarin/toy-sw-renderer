use crate::math::{BndBox2i, BndBox2f, Point2f, Point2i, Point3f, Vec2f, Vec3f, Vec3i};
use crate::renderer::Context;
use crate::tgaimage::{TGAColor, TGAImage};

#[allow(dead_code)]
pub fn draw_triangle_sweep(v1: Point2i, v2: Point2i, v3: Point2i, image: &mut TGAImage, color: TGAColor) {
    // handle degenerate triangle first
    if v1.y == v2.y && v2.y == v3.y {
        let xl = v1.x.min(v2.x.min(v3.x));
        let xr = v1.x.max(v2.x.max(v3.x));
        for x in xl..=xr {
            image.set(x, v1.y, color).unwrap();
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
            image.set(x, p1.y + i, color).unwrap();
        }
    }
}

pub fn draw_triangle_parallel(v1: Point2i, v2: Point2i, v3: Point2i, image: &mut TGAImage, color: TGAColor) {
    if v1.y == v2.y && v2.y == v3.y {
        let xl = v1.x.min(v2.x.min(v3.x));
        let xr = v1.x.max(v2.x.max(v3.x));
        for x in xl..=xr { image.set(x, v1.y, color).unwrap(); }
        return;
    } else if v1.x == v2.x && v2.x == v3.x {
        let yt = v1.y.min(v2.y.min(v3.y));
        let yb = v1.y.max(v2.y.max(v3.y));
        for y in yt..=yb { image.set(v1.x, y, color).unwrap(); }
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
                image.set(x, y, color).unwrap();
            }
        }
    }
}

pub fn draw_triangle(v1: Point2i, v2: Point2i, v3: Point2i, image: &mut TGAImage, color: TGAColor) {
    draw_triangle_parallel(v1, v2, v3, image, color);
}

fn barycentric(v1: &Point2f, v2: &Point2f, v3: &Point2f, p: &Point2f) -> Option<Point3f> {
    // solve linear eqn: p = 1 * v1 + u * v2 + v * v3;
    let aux1 = Vec3f { x: v2.x - v1.x, y: v3.x - v1.x, z: v1.x - p.x };
    let aux2 = Vec3f { x: v2.y - v1.y, y: v3.y - v1.y, z: v1.y - p.y };
    let solution = aux1.cross(aux2);
    if solution.z.abs() < 1e-2 {
        return None
    }
    Some(Point3f {
        x: 1.0 - (solution.x + solution.y) / solution.z,
        y: solution.x / solution.z,
        z: solution.y / solution.z,
    })
}

fn draw_3d_triangle_impl<C>(v1: Point3f,
                            v2: Point3f,
                            v3: Point3f,
                            ctx: &Context,
                            image: &mut TGAImage,
                            get_color: C,
                            z_buf: &mut [f32]) where C: Fn(f32, f32, f32) -> TGAColor {
    let iw = (image.width - 1) as f32;
    let ih = (image.height - 1) as f32;

    let local_v1 = ctx.camera.transform(&v1);
    let local_v2 = ctx.camera.transform(&v2);
    let local_v3 = ctx.camera.transform(&v3);

    // back face culling
    let rev_normal = (v3 - v1).cross(v2 - v1).normalize();
    let intensity = rev_normal.dot(ctx.light);
    if intensity <= 0.0 {
        return
    }

    let mut flat_v1 = local_v1.drop_z();
    flat_v1.x = (flat_v1.x + 0.5 * iw + 0.5).trunc();
    flat_v1.y = (flat_v1.y + 0.5 * ih + 0.5).trunc();
    let mut flat_v2 = local_v2.drop_z();
    flat_v2.x = (flat_v2.x + 0.5 * iw + 0.5).trunc();
    flat_v2.y = (flat_v2.y + 0.5 * ih + 0.5).trunc();
    let mut flat_v3 = local_v3.drop_z();
    flat_v3.x = (flat_v3.x + 0.5 * iw + 0.5).trunc();
    flat_v3.y = (flat_v3.y + 0.5 * ih + 0.5).trunc();

    let mut clamp = BndBox2f::new_empty();
    clamp.add_point(Point2f { x: 0.0, y: 0.0 });
    clamp.add_point(Point2f { x: iw, y: ih });
    let mut bbox = BndBox2f::new_empty();
    bbox.add_point(flat_v1);
    bbox.add_point(flat_v2);
    bbox.add_point(flat_v3);
    bbox.clamp_by(&clamp);

    let num_steps_x = (bbox.max.x - bbox.min.x) as i32;
    let num_steps_y = (bbox.max.y - bbox.min.y) as i32;
    for x_off in 0..=num_steps_x {
        for y_off in 0..=num_steps_y {
            let pnt = Point2f {
                x: bbox.min.x + x_off as f32,
                y: bbox.min.y + y_off as f32,
            };
            if let Some(bary) = barycentric(&flat_v1, &flat_v2, &flat_v3, &pnt) {
                let x = pnt.x as i32;
                let y = pnt.y as i32;
                let z = local_v1.z * bary.x +
                        local_v2.z * bary.y +
                        local_v3.z * bary.z;
                if bary.x < 0.0 || bary.y < 0.0 || bary.z < 0.0 {
                    continue;
                }

                if z_buf[(x + image.width * y) as usize] > z {
                    let shade = get_color(bary.x, bary.y, bary.z).scale(intensity);
                    z_buf[(x + image.width * y) as usize] = z;
                    image.set(x, y, shade).unwrap();
                }
            }
        }
    }
}

pub fn draw_3d_triangle(v1: Point3f,
                        v2: Point3f,
                        v3: Point3f,
                        ctx: &Context,
                        image: &mut TGAImage,
                        color: TGAColor,
                        z_buf: &mut [f32]) {
    let const_color = |_, _, _| {
        color
    };
    draw_3d_triangle_impl(v1, v2, v3, ctx, image, const_color, z_buf);
}

pub fn draw_3d_triangle_textured(v1: Point3f,
                                 v2: Point3f,
                                 v3: Point3f,
                                 tc1: Point2f,
                                 tc2: Point2f,
                                 tc3: Point2f,
                                 ctx: &Context,
                                 image: &mut TGAImage,
                                 diff_texture: &TGAImage,
                                 z_buf: &mut [f32]) {
    let diff_texture_picker = |l1, l2, l3| {
        let texpnt: Vec2f = l1 * <Point2f as Into<Vec2f>>::into(tc1) +
                            l2 * <Point2f as Into<Vec2f>>::into(tc2) +
                            l3 * <Point2f as Into<Vec2f>>::into(tc3);
        diff_texture.get((texpnt.x * diff_texture.width  as f32) as i32,
                         (texpnt.y * diff_texture.height as f32) as i32).unwrap()
    };

    draw_3d_triangle_impl(v1, v2, v3, ctx, image, diff_texture_picker, z_buf);
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
        let v2 = Point2i { x: 2, y: 3 };
        let v3 = Point2i { x: 5, y: 3 };

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

