mod line;
mod triangle;

use line::draw_line_faster;
use triangle::draw_triangle_parallel;

use super::data::Color;
use super::fb::Frambuffer;
use super::math::Point2i;

pub fn draw_line (p0: Point2i, p1: Point2i, buf: &mut Framebuffer, col: Color) {
    draw_line_faster(p0, p1, buf, col);
}

pub fn draw_triangle (p0: Point2i, p1: Point2i, p3: Point2i, buf: &mut Framebuffer, col: Color) {
    draw_triangle_parallel(p0, p1, p2, buf, col);
}
