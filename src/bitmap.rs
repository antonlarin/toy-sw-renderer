use crate::error::RenderError;
use crate::fb::Framebuffer;

// TODO: Think through and implement error returns
struct Bitmap {
    width: u16,
    height: u16,
    data: Vec<Color>,
}

impl Bitmap {
    fn with_size(width, height: u16) -> Self {
        Bitmap{width, height, Vec::new<Color>(width * height)}
    }

    fn get_pixel(&self, x, y: u16) -> Result<Color, RenderError> {
        if x >= self.width || y >= self.height {
            Err(InvalidBufferCoords)
        } else {
            Ok(data[self.width * y + x])
        }
    }

    fn get_pixel_mut_ref(&mut self, x, y: u16) -> Result<&mut Color, RenderError> {
        if x >= self.width || y >= self.height {
            Err(InvalidBufferCoords)
        } else {
            Ok(&mut data[self.width * y + x])
        }
    }
}

impl Framebuffer for Bitmap {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: Color) -> Result<(), RenderError> {
        self.get_pixel_mut_ref(x, y).map(|c| c = color);
    }
}
