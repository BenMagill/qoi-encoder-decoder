pub struct Header {
    pub magic_bytes: [char; 4], // "qoif"
    pub width: u32,
    pub height: u32,
    pub channels: u8, // 3 = RGB, 4 = RGBA
    pub colourspace: u8,
}

pub struct InputMetadata {
    pub width: u32,
    pub height: u32,
    pub channels: u8,
    pub colourspace: u8,
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Pixel {
    pub r: u8, 
    pub g: u8, 
    pub b: u8,
    pub a: u8
}