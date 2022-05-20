use crate::helpers;
use crate::structs;
use crate::constants;

pub fn exec(pixels: Vec<Vec<structs::Pixel>>, metadata: structs::InputMetadata) {
    println!("ENCODE!");

    let structs::InputMetadata {width, height, channels, colourspace} = metadata;
    let head = helpers::generate_header(width, height, channels, colourspace);

    let previously_seen: [structs::Pixel; 64] = [helpers::generate_empty_pixel(); 64];

    // stream is a vector of bytes that will be returned at the end
    let mut stream: Vec<u8> = vec!();
    // TODO add head struct as bytes to the stream

    let mut previous_value = structs::Pixel { r: 0, g: 0, b: 0, a: 255 };

    let mut current_run = 0;

    for row in pixels {
        for pixel in row {
            let index = helpers::hash(pixel);
            let dr = helpers::calulate_diff(pixel.r, previous_value.r, 2);
            let dg = helpers::calulate_diff(pixel.g, previous_value.g, 2);
            let db = helpers::calulate_diff(pixel.b, previous_value.b, 2);
            let dg_u6 = helpers::calulate_diff(pixel.g, previous_value.g, 32);
            let drdg = helpers::calulate_diff(
                helpers::calulate_diff(pixel.r, previous_value.r, 0),
                helpers::calulate_diff(pixel.g, previous_value.g, 0), 
                8
            );
            let dbdg = helpers::calulate_diff(
                helpers::calulate_diff(pixel.b, previous_value.b, 0),
                helpers::calulate_diff(pixel.g, previous_value.g, 0), 
                8
            );
            let alpha_changed = pixel.a - previous_value.a == 0;

            // GOI_OP_RUN
            //  number of times previous pixel repeats (from 1 to 62)
            if current_run == 63 || pixel != previous_value {
                // TODO need to store it now 

                current_run = 0
            } else if pixel == previous_value {
                current_run += 1;
            } else if previously_seen[index as usize] == pixel {
                // TODO store index and update index to be this value
            } else if alpha_changed && dr < 4 && dg < 4 && db < 4 {
                // QOI_OP_DIFF :
                //  can represent a difference in pixel colour between -2 and 1   
                // (alpha must be unchanged)
                let byte = 0b01000000 & dr << 4 & dg << 2 & db;
                println!("{:b}", byte);
                stream.push(byte);
            } else if alpha_changed && (dg_u6 < 64 && drdg < 16 && dbdg < 16) {
                // QOI_OP_LUMA
                //  green diff must be between -32 and 31
                //  red diff - green diff must be from -8 to 7
                //  blue diff - green diff must be from -8 to 7
                //  (alpha must be unchanged)
                let byte1 = 0b11000000 & (0b00111111 & dg_u6);
                let byte2 = (0b11110000 & drdg) & (0b00001111 & dbdg);
                stream.push(byte1);
                stream.push(byte2);
            } else if alpha_changed {
                // QOI_OP_RGB 
                // TODO
                //  use when alpha unchanged from previous pixel

            } else {
                // QOI_OP_RGBA
                // TODO
                //  use over QOI_OP_RGB when alpha value changed

            }

            previous_value = pixel;
        }
    }
}