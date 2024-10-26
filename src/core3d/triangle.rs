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

    // conversion from screen space into pixel coordinates
    // for X and Y: [-1, 1] -> [0, img_dim]
    let ones = Vec2f { x: 1.0, y: 1.0 };
    let img_half_dims = Point2f { x: 0.5 * iw, y: 0.5 * ih };
    let half_px_offset = Vec2f { x: 0.5, y: 0.5 };
    let flat_v1 = ((local_v1.drop_z() + ones) * img_half_dims + half_px_offset).trunc();
    let flat_v2 = ((local_v2.drop_z() + ones) * img_half_dims + half_px_offset).trunc();
    let flat_v3 = ((local_v3.drop_z() + ones) * img_half_dims + half_px_offset).trunc();

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
    let const_color = |_, _, _| { color };
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
