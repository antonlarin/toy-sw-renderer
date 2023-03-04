use core::result::Result;
use std::convert::From;
use std::fs::File;
use std::io::Write;

pub mod TGAFormat {
    pub const GRAYSCALE: i32 = 1;
    pub const RGB: i32 = 3;
    pub const RGBA: i32 = 4;
}

#[derive(Debug)]
pub enum TGAError {
    EmptyImage,
    InvalidCoords,
    FileOpenError,
    WriteError
}

type TGAResult<T> = Result<T, TGAError>;

impl From<std::io::Error> for TGAError {
    fn from(err: std::io::Error) -> TGAError {
        TGAError::WriteError
    }
}

#[repr(align(1))]
struct TGAHeader {
    id_length: i8,
    color_map_type: i8,
    data_type_code: i8,
    color_map_origin: i16,
    color_map_length: i16,
    color_map_depth: i8,
    x_origin: i16,
    y_origin: i16,
    width: i16,
    height: i16,
    bits_per_pixel: i8,
    image_descriptor: i8
}

impl TGAHeader {
    fn from_image(img: &TGAImage) -> TGAHeader {
        TGAHeader {
            id_length: 0,
            color_map_type: 0,
            data_type_code: if img.bytespp == TGAFormat::GRAYSCALE { 11 } else { 10 },
            color_map_origin: 0,
            color_map_length: 0,
            color_map_depth: 0,
            x_origin: 0,
            y_origin: 0,
            width: img.width as i16,
            height: img.height as i16,
            bits_per_pixel: (img.bytespp * 8) as i8,
            image_descriptor: 0x20,
        }
    }

    fn write<W: Write>(self: &Self, sink: &mut W) -> TGAResult<()> {
        let mut res: Vec<u8> = Vec::with_capacity(std::mem::size_of::<TGAHeader>());
        res.push(self.id_length as u8);
        res.push(self.color_map_type as u8);
        res.push(self.data_type_code as u8);
        res.extend_from_slice(&self.color_map_origin.to_le_bytes());
        res.extend_from_slice(&self.color_map_length.to_le_bytes());
        res.push(self.color_map_depth as u8);
        res.extend_from_slice(&self.x_origin.to_le_bytes());
        res.extend_from_slice(&self.y_origin.to_le_bytes());
        res.extend_from_slice(&self.width.to_le_bytes());
        res.extend_from_slice(&self.height.to_le_bytes());
        res.push(self.bits_per_pixel as u8);
        res.push(self.image_descriptor as u8);

        sink.write(&res)?;
        Ok(())
    }
}

#[derive(Debug)]
pub struct TGAColor {
    val: [u8; 4],
    bytespp: i32
}

impl TGAColor {
    pub fn new() -> TGAColor {
        TGAColor {
            val: [0; 4],
            bytespp: 1
        }
    }

    pub fn from_components(r: u8, g: u8, b: u8, a: u8) -> TGAColor {
        TGAColor {
            val: [b, g, r, a],
            bytespp: 1
        }
    }

    pub fn from_packed_components(v: i32, bpp: i32) -> TGAColor {
        let uv = v as u32;
        let b = ((uv >> 24) & 0xFF) as u8;
        let g = ((uv >> 16) & 0xFF) as u8;
        let r = ((uv >>  8) & 0xFF) as u8;
        let a = ( uv        & 0xFF) as u8;
        TGAColor {
            val: [b, g, r, a],
            bytespp: bpp
        }
    }

    pub fn from_component_slice(s: &[u8], bpp: i32) -> TGAColor {
        let mut values: [u8; 4] = [0; 4];
        for i in 0..bpp {
            values[i as usize] = s[i as usize];
        }

        TGAColor {
            val: values,
            bytespp: bpp
        }
    }
}

#[derive(Debug)]
pub struct TGAImage {
    data: Vec<u8>,
    width: i32,
    height: i32,
    bytespp: i32
}

// fn load_rle_data() {

// }

fn unload_rle_data<W: Write>(img: &TGAImage, sink: &mut W) -> TGAResult<()> {
    const MAX_CHUNK: i32 = 128;
    let num_pixels = img.width * img.height;
    let mut current_px = 0;

    while current_px < num_pixels {
        let chunk_start = current_px * img.bytespp;
        let current_byte = current_px * img.bytespp;
        let mut run_length = 1;
        let is_raw = true;

        while current_px + run_length < num_pixels && run_length < MAX_CHUNK {
            let next_equal = true;

            todo!();

            run_length += 1;
        }
    }

    Ok(())
}

impl TGAImage {
    pub fn new() -> Self {
        TGAImage {
            data: Vec::new(),
            width: 0,
            height: 0,
            bytespp: 0
        }
    }

    pub fn with_size(w: i32, h: i32, bpp: i32) -> Self {
        let size = w * h * bpp;
        let data = vec![0 as u8; size as usize];
        TGAImage {
            data: data,
            width: w,
            height: h,
            bytespp: bpp
        }
    }

    // fn from_tga_file(filename: &str) -> Option<Self> {

    // }

    pub fn write_to_file(self: &Self, filename: &str) -> TGAResult<()> {
        const FOOTER: &str = "\0\0\0\0\0\0\0\0TRUEVISION-XFILE.\0";

        if let Ok(mut file) = File::create(filename) {
            let header = TGAHeader::from_image(self);
            header.write(&mut file)?;
            unload_rle_data(self, &mut file)?;
            file.write(FOOTER.as_bytes())?;
        } else {
            return Err(TGAError::FileOpenError)
        }

        Ok(())
    }

    // fn flip_horizontally(self: &mut Self) -> bool {

    // }

    pub fn flip_vertically(self: &mut Self) -> TGAResult<()> {
        if self.data.is_empty() {
            return Err(TGAError::EmptyImage)
        }

        let bytes_per_line = (self.width * self.bytespp) as usize;
        let mut line1: Vec<u8> = Vec::with_capacity(bytes_per_line);
        let mut line2: Vec<u8> = Vec::with_capacity(bytes_per_line);

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

    // fn scale(self: &mut Self, w: i32, h: i32) -> bool {

    // }

    pub fn get(self: &Self, x: i32, y: i32) -> TGAResult<TGAColor> {
        if self.data.is_empty() {
            return Err(TGAError::EmptyImage)
        } else if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return Err(TGAError::InvalidCoords)
        }

        let offset = (x + y * self.width) as usize;
        let pixel = &self.data.as_slice()[offset..(offset + self.bytespp as usize)];

        Ok(TGAColor::from_component_slice(pixel, self.bytespp))
    }

    pub fn set(self: &mut Self, x: i32, y: i32, c: TGAColor) -> TGAResult<()> {
        if self.data.is_empty() {
            return Err(TGAError::EmptyImage)
        } else if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return Err(TGAError::InvalidCoords)
        }

        let bpp = self.bytespp as usize;
        let offset = (x + y * self.width) as usize;
        self.data[offset..(offset + bpp)].copy_from_slice(c.val[0..bpp].as_ref());

        Ok(())
    }

    // fn buffer(self: &Self) -> &[u8] {
    // }

    fn clear(self: &mut Self) {
        self.data.fill(0 as u8);
    }
}
