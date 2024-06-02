pub fn draw_triangle(v1: Point2, v2: Point2, v3: Point2, image: &mut TGAImage, color: TGAColor) {
    draw_line(v1.x as i32, v1.y as i32, v2.x as i32, v2.y as i32, image, color);
    draw_line(v2.x as i32, v2.y as i32, v3.x as i32, v3.y as i32, image, color);
    draw_line(v3.x as i32, v3.y as i32, v1.x as i32, v1.y as i32, image, color);

    // TODO: fill in the triangle.
    // Idea:
    // Sort vertices of the triangle by their y-coordinates;
    // Rasterize simultaneously the left and the right sides of the triangle;
    // Draw a horizontal line segment between the left and the right boundary points.
    let mut vs = [v1, v2, v3];
    vs.sort_by(|p1, p2| p1.y.total_cmp(&p2.y));

    let (top, left, right, yinc) = if vs[0].y as i32 == vs[1].y as i32 {
        (vs[2], vs[0], vs[1], -1)
    } else {
        (vs[0], vs[1], vs[2], 1)
    };

    if left.x > right.x {
        std::mem::swap(&mut left, &mut right);
    }

    // TODO: iterate over y from top.y to min(left.y, right.y)
    //       compute xs on top-left and top-right at y, draw a line between these xs
    //       then iterate from min(left.y, right.y) to max(left.y, right.y)
    //       compute xs on remaining two bounding lines, draw a line between xs
}

