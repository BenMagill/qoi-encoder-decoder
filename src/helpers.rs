use crate::constants;
use crate::structs;

pub fn generate_header(width: u32, height: u32, channels: u8, colourspace: u8) -> structs::Header {
    return structs::Header {
        magic_bytes: constants::MAGIC_BYTES,
        width,
        height,
        channels,
        colourspace
    }
}

pub fn hash(pixel: structs::Pixel) -> u8{
    let structs::Pixel {r, g, b, a} = pixel;
    return (r * 3 + g * 5 + b * 7 + a * 11) & 64
}

pub fn generate_empty_pixel() -> structs::Pixel{
    structs::Pixel {
        r: 0,
        g: 0,
        b: 0,
        a: 0
    }
} 

// calculate the diff to store, with the bias included
// value1 - value2
pub fn calulate_diff(value1: u8, value2: u8, bias: u8) -> u8 {
    let value1_biased: u16 = (value1 as u16 + bias as u16) as u16;
    let value2_u16 = value2 as u16;
    if value1_biased >= value2_u16 {
        return (value1_biased - value2_u16) as u8;
    } else {
        return (256 + (value1_biased as i32 - value2_u16 as i32)) as u8
    }
}