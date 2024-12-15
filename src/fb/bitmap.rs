use crate::data::Color;
use crate::error::SwrenderError;
use crate::fb::Framebuffer;

pub struct Bitmap {
    width: u16,
    height: u16,
    data: Vec<u8>,
}

// Use RGB, aligned with data::Color
const BYTES_PER_PIXEL: usize = 3;

impl Bitmap {
    pub fn with_size(width: u16, height: u16) -> Self {
        let data_size = (width * height) as usize * BYTES_PER_PIXEL;
        Bitmap{width, height, data: vec![0u8; data_size]}
    }

    fn get_pixel(&self, x: u16, y: u16) -> Result<Color, SwrenderError> {
        if x >= self.width || y >= self.height {
            Err(SwrenderError::InvalidPixelCoords)
        } else {
            let offset = (self.width * y + x) as usize * BYTES_PER_PIXEL;
            Ok(Color{
                r: self.data[offset],
                g: self.data[offset + 1],
                b: self.data[offset + 2],
            })
        }
    }

    fn get_pixel_as_mut_slice(&mut self, x: u16, y: u16) -> Result<&mut [u8], SwrenderError> {
        if x >= self.width || y >= self.height {
            Err(SwrenderError::InvalidPixelCoords)
        } else {
            let offset = (self.width * y + x) as usize * BYTES_PER_PIXEL;
            Ok(&mut self.data[offset..offset + BYTES_PER_PIXEL])
        }
    }

    pub fn flip_vertically(&mut self) -> Result<(), SwrenderError> {
        let bytes_per_line = (self.width * self.bytespp) as usize;
        let mut line1 = vec![0_u8; bytes_per_line];
        let mut line2 = vec![0_u8; bytes_per_line];

        let half = (self.height / 2) as usize;
        for i in 0..half {
            let l1 = i * bytes_per_line;
            let l2 = ((self.height - 1 - i as i32) * bytes_per_line as i32) as usize;

            {
                let l1_slice = &self.data.as_slice()[l1..(l1 + bytes_per_line)];
                line1.as_mut_slice().copy_from_slice(l1_slice);
                let l2_slice = &self.data.as_slice()[l2..(l2 + bytes_per_line)];
                line2.as_mut_slice().copy_from_slice(l2_slice);
            }

            {
                let l1_slice = &mut self.data.as_mut_slice()[l1..(l1 + bytes_per_line)];
                l1_slice.copy_from_slice(line2.as_slice());
                let l2_slice = &mut self.data.as_mut_slice()[l2..(l2 + bytes_per_line)];
                l2_slice.copy_from_slice(line1.as_slice());
            }
        }

        Ok(())
    }

}

impl Framebuffer for Bitmap {
    fn width(&self) -> i32 {
        self.width
    }

    fn height(&self) -> i32 {
        self.height
    }

    fn set_pixel(&mut self, x: i32, y: i32, color: Color) -> Result<(), SwrenderError> {
        self.get_pixel_mut_ref(x, y).map(|c| c = color)
    }
}
