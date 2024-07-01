use core::result::Result;
use std::convert::From;
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};

pub mod tga_format {
    pub const GRAYSCALE: i32 = 1;
    pub const RGB: i32 = 3;
    pub const RGBA: i32 = 4;
}

#[derive(Debug)]
pub enum TGAError {
    EmptyImage,
    InvalidCoords(i32, i32),
    FileOpenError,
    WriteError
}

type TGAResult<T> = Result<T, TGAError>;

impl From<std::io::Error> for TGAError {
    fn from(_: std::io::Error) -> TGAError {
        TGAError::WriteError
    }
}

#[repr(align(1))]
#[derive(Debug)]
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
    const HEADER_LEN: usize = 18;

    fn from_image(img: &TGAImage) -> TGAHeader {
        TGAHeader {
            id_length: 0,
            color_map_type: 0,
            data_type_code: if img.bytespp == tga_format::GRAYSCALE { 11 } else { 10 },
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

    fn from_bytes(buf: &[u8]) -> TGAHeader {
        assert!(buf.len() == Self::HEADER_LEN);
        let slice_to_i16 = |rng: std::ops::Range<usize>| {
            i16::from_le_bytes(buf[rng].try_into().unwrap())
        };
        TGAHeader {
            id_length: buf[0] as i8,
            color_map_type: buf[1] as i8,
            data_type_code: buf[2] as i8,
            color_map_origin: slice_to_i16(3..5),
            color_map_length: slice_to_i16(5..7),
            color_map_depth: buf[7] as i8,
            x_origin: slice_to_i16(8..10),
            y_origin: slice_to_i16(10..12),
            width: slice_to_i16(12..14),
            height: slice_to_i16(14..16),
            bits_per_pixel: buf[16] as i8,
            image_descriptor: buf[17] as i8,
        }
    }

    fn write<W: Write>(self, sink: &mut W) -> TGAResult<()> {
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

        sink.write_all(&res)?;
        Ok(())
    }

    fn read<R: Read>(source: &mut R) -> TGAResult<TGAHeader> {
        let mut buf = [0u8; Self::HEADER_LEN];
        source.read_exact(&mut buf)?;
        let header = TGAHeader::from_bytes(&buf);
        Ok(header)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct TGAColor {
    val: [u8; 4],
    bytespp: i32,
}

impl TGAColor {
    pub fn from_rgb(r: u8, g: u8, b: u8) -> TGAColor {
        TGAColor {
            val: [b, g, r, 0],
            bytespp: 3,
        }
    }
    pub fn from_rgba(r: u8, g: u8, b: u8, a: u8) -> TGAColor {
        TGAColor {
            val: [b, g, r, a],
            bytespp: 4,
        }
    }

    pub fn from_packed_components(v: i32) -> TGAColor {
        let uv = v as u32;
        let b = ((uv >> 24) & 0xFF) as u8;
        let g = ((uv >> 16) & 0xFF) as u8;
        let r = ((uv >>  8) & 0xFF) as u8;
        let a = ( uv        & 0xFF) as u8;
        TGAColor {
            val: [b, g, r, a],
            bytespp: 4,
        }
    }

    pub fn from_component_slice(s: &[u8], bpp: i32) -> TGAColor {
        let mut values: [u8; 4] = [0; 4];
        for i in 0..bpp {
            values[i as usize] = s[i as usize];
        }

        TGAColor {
            val: values,
            bytespp: bpp,
        }
    }

    pub fn r(&self) -> u8 { self.val[2] }
    pub fn g(&self) -> u8 { self.val[1] }
    pub fn b(&self) -> u8 { self.val[0] }
    pub fn a(&self) -> u8 { self.val[3] }

    pub fn scale(&self, factor: f32) -> Self {
        let mut res_val = [0u8; 4];
        for i in 0..4 {
            res_val[i] = (self.val[i] as f32 * factor) as u8;
        }
        Self { val: res_val, bytespp: self.bytespp }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct TGAImage {
    data: Vec<u8>,
    pub width: i32,
    pub height: i32,
    bytespp: i32
}

fn load_rle_data<R: Read>(src: &mut R, img: &mut TGAImage) ->TGAResult<()> {
    Ok(())
}

fn unload_rle_data<W: Write>(img: &TGAImage, dst: &mut W) -> TGAResult<()> {
    const MAX_CHUNK: usize = 128;
    let num_pixels = (img.width * img.height) as usize;
    let mut next_px_to_write = 0_usize;

    let data = img.data.as_slice();
    let bpp = img.bytespp as usize;

    while next_px_to_write < num_pixels {
        let chunk_start = next_px_to_write * bpp;
        let mut i = next_px_to_write * bpp;
        let mut run_length = 1;
        let mut run_raw = true;

        while next_px_to_write + run_length < num_pixels && run_length < MAX_CHUNK {
            let next_pair_equal = data[i..(i + bpp)] == data[(i + bpp)..(i + 2 * bpp)];
            i += bpp;

            // when starting new run, determine if it'll be raw run or
            // rl-encoded run
            if run_length == 1 {
                run_raw = !next_pair_equal;
            }

            // when in the middle of the run, check end conditions
            if run_raw && next_pair_equal {
                // break raw run only if three equal pixels are found
                if next_px_to_write + run_length + 1 < num_pixels &&
                   data[i..(i + bpp)] == data[(i + bpp)..(i + 2 * bpp)] {
                    // if raw run is broken, current pixel belongs to
                    // the next encoded run
                    run_length -= 1;
                    break;
                }
            }
            if !run_raw && !next_pair_equal {
                break;
            }

            // if run continues, the next pixel we checked just now is a part
            // of it, so increment the length
            run_length += 1;
        }
        next_px_to_write += run_length;

        // subtracting 1 in both cases so that marker range [0, 127] corresponds to
        // run length range [1, 128] to utilize all values, since zero-length
        // runs don't exist
        let marker = if run_raw {
            run_length - 1
        } else {
            (run_length - 1) | 0x80
        };

        dst.write_all(&[marker as u8])?;
        if run_raw {
            dst.write_all(&data[chunk_start..(chunk_start + run_length * bpp)])?;
        } else {
            dst.write_all(&data[chunk_start..(chunk_start + bpp)])?;
        }
    }

    Ok(())
}

impl TGAImage {
    const FOOTER: &str = "\0\0\0\0\0\0\0\0TRUEVISION-XFILE.\0";

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
        let data = vec![0_u8; size as usize];
        TGAImage {
            data,
            width: w,
            height: h,
            bytespp: bpp
        }
    }

    pub fn from_tga_file(filename: &str) -> TGAResult<Self> {
        if let Ok(file) = File::open(filename) {
            let mut buffered_file = BufReader::new(file);
            let header = TGAHeader::read(&mut buffered_file)?;
            println!("Read in TGAHeader {:?}", header);
            let mut image = Self::with_size(header.width.into(),
                                            header.height.into(),
                                            header.bits_per_pixel as i32 / 8);
            load_rle_data(&mut buffered_file, &mut image)?;
            // TODO: check that footer is present
            Ok(image)
        } else {
            return Err(TGAError::FileOpenError)
        }
    }

    pub fn write_to_file(&self, filename: &str) -> TGAResult<()> {
        if let Ok(file) = File::create(filename) {
            let mut buffered_file = BufWriter::new(file);
            let header = TGAHeader::from_image(self);
            header.write(&mut buffered_file)?;
            unload_rle_data(self, &mut buffered_file)?;
            buffered_file.write_all(Self::FOOTER.as_bytes())?;
        } else {
            return Err(TGAError::FileOpenError)
        }

        Ok(())
    }

    // fn flip_horizontally(self: &mut Self) -> bool {

    // }

    pub fn flip_vertically(&mut self) -> TGAResult<()> {
        if self.data.is_empty() {
            return Err(TGAError::EmptyImage)
        }

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

    // fn scale(self: &mut Self, w: i32, h: i32) -> bool {

    // }

    pub fn get(&self, x: i32, y: i32) -> TGAResult<TGAColor> {
        if self.data.is_empty() {
            return Err(TGAError::EmptyImage)
        } else if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return Err(TGAError::InvalidCoords(x, y))
        }

        let bpp = self.bytespp as usize;
        let offset = (x + y * self.width) as usize * bpp;
        let pixel = &self.data.as_slice()[offset..(offset + bpp)];

        Ok(TGAColor::from_component_slice(pixel, self.bytespp))
    }

    pub fn set(&mut self, x: i32, y: i32, c: TGAColor) -> TGAResult<()> {
        if self.data.is_empty() {
            return Err(TGAError::EmptyImage)
        } else if x < 0 || y < 0 || x >= self.width || y >= self.height {
            return Err(TGAError::InvalidCoords(x, y))
        }

        let bpp = self.bytespp as usize;
        let offset = (x + y * self.width) as usize * bpp;
        self.data[offset..(offset + bpp)].copy_from_slice(&c.val[..bpp]);

        Ok(())
    }

    // TODO: think about it some more
    /*
    pub fn set_in_block<S>(&mut self, x0: i32, x1: i32, y0: i32, y1: i32, setter: S)
    where S: Fn(i32, i32, &mut [u8]) {
        let mut data = self.data.as_mut_slice();
        let line_block_start = (y0 * self.width) as usize;
        let line_block_end = ((y1 + 1) * self.width) as usize;
        data[line_block_start..line_block_end].par_iter_mut()
                                              .enumerate()
                                              .map(|(i, v)| {
                                                  (i as i32 % self.width,
                                                   i as i32 / self.width + y0,
                                                   v)
                                              })
                                              .filter(|(x, y, v)| x >= x0 && x <= x1)
                                              .for_each;
    }
    */

    pub fn clear(&mut self) {
        self.data.fill(0_u8);
    }
}

#[cfg(test)]
mod tests {
    use super::{tga_format, TGAColor, TGAImage, unload_rle_data};

    #[test]
    fn rle_encode_grayscale() {
        let image = TGAImage {
            data: vec![0, 3, 1, 4, 4, 5, 5, 5, 0],
            width: 3,
            height: 3,
            bytespp: tga_format::GRAYSCALE
        };

        let mut target = vec![0_u8; 10];
        unload_rle_data(&image, &mut target.as_mut_slice()).unwrap();

        let expected = [4, 0, 3, 1, 4, 4, 130, 5, 0, 0] as [u8; 10];
        assert_eq!(target.as_slice(), expected);
    }

    #[test]
    fn rle_encode_rgb() {
        let image = TGAImage {
            data: vec![
                0, 0, 0,   0,   0, 0,   0, 0, 0,
                0, 0, 0,   255, 0, 0,   0, 0, 0,
                0, 0, 0,   0,   0, 0,   0, 0, 0
            ],
            width: 3,
            height: 3,
            bytespp: tga_format::RGB
        };

        let mut target = vec![0_u8; 12];
        unload_rle_data(&image, &mut target.as_mut_slice()).unwrap();

        let expected = [131, 0, 0, 0, 0, 255, 0, 0, 131, 0, 0, 0];
        assert_eq!(target.as_slice(), expected);
    }

    #[test]
    fn flip_vertically() {
        let mut image = TGAImage {
            data: vec![1, 2, 3, 4, 5, 6],
            width: 3,
            height: 2,
            bytespp: tga_format::GRAYSCALE
        };

        image.flip_vertically().unwrap();

        let expected = [4, 5, 6, 1, 2, 3] as [u8; 6];
        assert_eq!(image.data.as_slice(), expected);
    }

    #[test]
    fn set() {
        let mut image = TGAImage::with_size(5, 5, tga_format::RGBA);
        image.set(1, 2, TGAColor::from_rgba(255, 0, 0, 255)).unwrap();
        assert_eq!(image.data[46], 255);
    }

    #[test]
    fn get() {
        let image = TGAImage {
            data: vec![
                0, 0, 0,   0, 0, 0,
                0, 0, 0,   0, 0, 0,
                0, 0, 0,   128, 0, 0
            ],
            width: 2,
            height: 3,
            bytespp: tga_format::RGB
        };

        let color = image.get(1, 2).unwrap();
        assert_eq!(color.r(), 0);
        assert_eq!(color.g(), 0);
        assert_eq!(color.b(), 128);
    }
}
