enum Format {
    Grayscale = 1,
    RGB = 3,
    RGBA = 4,
}

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
        return TGAColor {
            val: [0; 4],
            bytespp: 1
        }
    }

    pub fn from_components(r: u8, g: u8, b: u8, a: u8) -> TGAColor {
        return TGAColor {
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
        return TGAColor {
            val: [b, g, r, a],
            bytespp: bpp
        }
    }

    pub fn from_component_array(s: &[u8], bpp: i32) -> TGAColor {
        let mut values: [u8; 4] = [0; 4];
        for i in 0..bpp {
            values[i as usize] = s[i as usize];
        }

        return TGAColor {
            val: values,
            bytespp: bpp
        }
    }
}

#[derive(Debug)]
struct TGAImage {
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

    // fn new_with_size(w: i32, h: i32, bpp: i32) -> Self {

    // }

    // fn from_tga_file(filename: &str) -> Option<Self> {

    // }

    // fn write_to_file(filename: &str, rle: bool) -> bool {

    // }

    // fn flip_horizontally(self: &mut Self) -> bool {

    // }

    // fn flip_vertically(self: &mut Self) -> bool {

    // }

    // fn scale(self: &mut Self, w: i32, h: i32) -> bool {

    // }

    // fn get(self: &Self, x: i32, y: i32) -> TGAColor {

    // }

    // fn set(self: &mut Self, x: i32, y: i32, c: TGAColor) -> bool {

    // }

    // fn get_width(self: &Self) -> i32 {

    // }

    // fn get_height(self: &Self) -> i32 {

    // }

    // fn get_bytespp(self: &Self) -> i32 {

    // }

    // fn buffer(self: &Self) -> &[u8] {
    // }

    fn clear(self: &mut Self) {

    }
}
