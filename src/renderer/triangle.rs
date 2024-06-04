use crate::math::Point2i;
use crate::tgaimage::{TGAColor, TGAImage};
use super::draw_line;

pub fn draw_triangle(v1: Point2i, v2: Point2i, v3: Point2i, image: &mut TGAImage, color: TGAColor) {
    draw_line(v1.x, v1.y, v2.x, v2.y, image, color);
    draw_line(v2.x, v2.y, v3.x, v3.y, image, color);
    draw_line(v3.x, v3.y, v1.x, v1.y, image, color);

    if v1.y == v2.y && v2.y == v3.y {
        draw_line(v1.x.min(v2.x.min(v3.x)), v1.y, v1.x.max(v2.x.max(v3.x)), v1.y, image, color);
        return
    }

    let mut vs = [v1, v2, v3];
    vs.sort_by_key(|p| p.y);

    let upside_down = vs[0].y == vs[1].y;
    let (mut apex, mut left, mut right) = if upside_down {
        (vs[2], vs[0], vs[1])
    } else {
        (vs[0], vs[1], vs[2])
    };
    if left.x > right.x {
        std::mem::swap(&mut left, &mut right);
    }

    // draw first half of the triangle: 'apex' to 'closest to apex by y from left, right'
    let apex_range = if upside_down { left.y.max(right.y)..=apex.y } else { apex.y..=left.y.min(right.y) };
    for y in apex_range {
        let left_x = apex.x + (left.x - apex.x) * (y - apex.y) / (left.y - apex.y);
        let right_x = apex.x + (right.x - apex.x) * (y - apex.y) / (right.y - apex.y);

        draw_line(left_x, y, right_x, y, image, color);
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

        draw_line(left_x, y, right_x, y, image, color);
    }
}

#[cfg(test)]
mod test {
    // TODO:
    // * add unit tests
    // * add benchmark? gonna need nightly rust though
}

