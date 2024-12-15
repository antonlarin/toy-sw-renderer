mod bitmap;

use crate::data::Color;
use crate::error::SwrenderError;

pub trait Framebuffer {
    fn width(&self) -> i32;
    fn height(&self) -> i32;

    fn set_pixel(&mut self, x: i32, y: i32, col: Color) -> Result<(), SwrenderError>;
}

pub use bitmap::Bitmap;
