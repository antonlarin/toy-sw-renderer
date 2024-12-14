mod line;
mod triangle;

use line::draw_line_fast;
use triangle::draw_triangle_parallel;

use super::data::Color;
use super::fb::Framebuffer;
use super::math::Point2i;

pub fn draw_line<FB: Framebuffer>(p0: Point2i, p1: Point2i, buf: &mut FB, col: Color) {
    draw_line_fast(p0, p1, buf, col);
}

pub fn draw_triangle<FB: Framebuffer>(p0: Point2i, p1: Point2i, p2: Point2i, buf: &mut FB, col: Color) {
    draw_triangle_parallel(p0, p1, p2, buf, col);
}
