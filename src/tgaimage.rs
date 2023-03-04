pub const GRAYSCALE: i32 = 1;
pub const RGB: i32 = 3;
pub const RGBA: i32 = 4;

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

    pub fn from_component_array(s: &[u8], bpp: i32) -> TGAColor {
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

fn load_rle_data() {

}

fn unload_rle_data() {

}

impl TGAImage {
    // fn new() -> Self {

    // }

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

    pub fn write_to_file(self: &Self, filename: &str) -> bool {

    }

    // fn flip_horizontally(self: &mut Self) -> bool {

    // }

    pub fn flip_vertically(self: &mut Self) -> bool {
        if self.data.is_empty() {
            return false
        }

        let bytes_per_line = (self.width * self.bytespp) as usize;
        let mut line: Vec<u8> = Vec::with_capacity(bytes_per_line);

        let half = (self.height / 2) as usize;
        for i in 0..half {
            let l1 = i * bytes_per_line;
            let l2 = ((self.height - 1 - i as i32) * bytes_per_line as i32) as usize;

            let l1_slice = &mut self.data.as_mut_slice()[l1..(l1 + bytes_per_line)];
            let l2_slice = &mut self.data.as_mut_slice()[l2..(l2 + bytes_per_line)];

            line.as_mut_slice().copy_from_slice(l1_slice);
            l1_slice.copy_from_slice(l2_slice);
            l2_slice.copy_from_slice(line.as_slice());
        }

        return true
    }

    // fn scale(self: &mut Self, w: i32, h: i32) -> bool {

    // }

    // fn get(self: &Self, x: i32, y: i32) -> TGAColor {

    // }

    pub fn set(self: &mut Self, x: i32, y: i32, c: TGAColor) -> bool {
        if self.data.is_empty() || x < 0 || y < 0 || x >= self.width || y >= self.height {
            return false
        }

        let bpp = self.bytespp as usize;
        let offset = (x + y * self.width) as usize;
        self.data[offset..(offset + bpp)].copy_from_slice(c.val[0..bpp].as_ref());

        true
    }

    // fn buffer(self: &Self) -> &[u8] {
    // }

    fn clear(self: &mut Self) {

    }
}
