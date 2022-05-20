pub const MAGIC_BYTES: [char; 4] = ['q','o','i','f'];

pub const RGB_CHUNK_ID: u8 = 0b11111110;
pub const RGBA_CHUNK_ID: u8 = 0b11111111;

pub const STREAM_END: [u8; 8] = [
    0b0,
    0b0,
    0b0,
    0b0,
    0b0,
    0b0,
    0b0,
    0b1,
];

pub mod COLOURSPACE {
    pub const SRGB: u8 = 0;
    pub const LINEAR: u8 = 0;
}

pub mod CHANNELS {
    pub const RGB: u8 = 3;
    pub const RGBA: u8 = 4;
}