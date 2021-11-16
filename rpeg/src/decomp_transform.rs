use std::io::Read;
use std::vec;
use array2::Array2;
use csc411_arith::chroma_of_index;
use csc411_image::{Image, Pixel, Rgb};
use csc411_image;
use bitpack::bitpack;
use scan_fmt::scan_fmt;

#[derive(Debug,Clone,Copy)]
pub struct ImageRgb {
    r: f32,
    g: f32,
    b: f32,
}

#[derive(Debug,Clone,Copy)]
pub struct ImageVid {
    y: f32,
    pb: f32,
    pr: f32,
}

#[derive(Debug,Clone,Copy)]
pub struct ImageCos {
    indexed_pb: usize,
    indexed_pr: usize,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}
///  Reads through the compressed image and returns an "array2" of 64 bit words, with 
///  the lower 32 bits being occupied by our ImageCos values. Returns an "array2" struct of those words.
///
/// # Arguments:
/// * `filename`: The file containing the compressed ppm image
pub fn get_compressed(filename: &str) -> Array2<u64> {
    let mut word_vec: Vec<u64> = vec![];
    let mut reader: Box<dyn std::io::BufRead> = Box::new(
        std::io::BufReader::new(std::fs::File::open(filename).unwrap(),
        ),
    );
    let mut header = String::new();
    let mut buf: Vec<u8> = Vec::new();
    reader.read_line(&mut header).unwrap();
    reader.read_line(&mut header).unwrap();
    let (w,h) = scan_fmt!(&header, "Compressed image format 2\n{} {}", u32, u32).unwrap();
    reader.read_to_end(&mut buf).unwrap();
    let mut curr_word_array: [u8; 4] = [0; 4];
    for i in (0..buf.len()).step_by(4) {
        curr_word_array[0] = buf[i];
        curr_word_array[1] = buf[i+1];
        curr_word_array[2] = buf[i+2];
        curr_word_array[3] = buf[i+3];
        let word = u32::from_be_bytes(curr_word_array) as u64;
        word_vec.push(word);
    }
    let word_array2 = array2::Array2::from_row_major((w / 2) as usize,(h / 2) as usize,word_vec).unwrap();
    return word_array2;
    
    
}
/// goes through each word in the "word_array" and unpacks the ImageCos values.
/// returns those values (a, b, c, d, avg pb, avg pr) in an "array2" struct of ImageCos.
///
/// # Arguments:
/// * `word_array`: the "array2" containing our words that were packed previously.
pub fn word_to_cos (word_array: Array2<u64>) -> Array2<ImageCos> {
    let mut cos_vec:Vec<ImageCos> = vec![];
    for i in word_array.iter_row_major() {
        let current_word = *i.2;
        let current_block = unpack_word(current_word);
        cos_vec.push(current_block);
    }
    let cos_array2 = array2::Array2::from_row_major(word_array.width(), word_array.height(), cos_vec).unwrap();
    return cos_array2;
}

/// Iterates through the "transformed_array" and extrapolates the values of each pixel in the 2x2 block
/// back to values close to their original. returns an "array2" struct of our component video values.
///
/// # Arguments:
/// * `transformed_array`: The "array2" of ImageCos values.
pub fn reverse_block(transformed_array: Array2<ImageCos>) -> Array2<ImageVid>{
    let mut counter = 0;
    let width = transformed_array.width() * 2;
    let height = transformed_array.height() * 2;
    let blank_struct = ImageVid {y: 0.0 , pb:0.0, pr: 0.0};
    let mut working_vec: Vec<ImageVid> = vec![blank_struct; width * height];
    for i in transformed_array.iter_row_major() {
        if counter == 0 {
            counter = 0;
        } else if counter % width == 0 {
            counter += width;
        }
        let curr_block = i.2;
        let a = curr_block.a / 511.0;
        let b = curr_block.b / 50.0;
        let c = curr_block.c / 50.0;
        let d = curr_block.d / 50.0;
        let y1 = a - b - c + d;
        let y2 = a - b + c - d;
        let y3 = a + b - c - d;
        let y4 = a + b + c + d;
        let avg_pb = chroma_of_index(curr_block.indexed_pb);
        let avg_pr = chroma_of_index(curr_block.indexed_pr);
        let curr_pixel1 = ImageVid {
            y: y1,
            pb: avg_pb,
            pr: avg_pr,
        };
        let curr_pixel2 = ImageVid {
            y: y2,
            pb: avg_pb,
            pr: avg_pr,
        };
        let curr_pixel3 = ImageVid {
            y: y3,
            pb: avg_pb,
            pr: avg_pr,
        };
        let curr_pixel4 = ImageVid {
            y: y4,
            pb: avg_pb,
            pr: avg_pr,      
        };
        
        working_vec[counter] = curr_pixel1; 
        working_vec[counter + 1] = curr_pixel2;
        working_vec[counter + width] = curr_pixel3;
        working_vec[counter + width + 1] = curr_pixel4;
        
        counter += 2;
    }
    let vid_array2 = array2::Array2::from_row_major(width, height, working_vec).unwrap();
    return vid_array2;
 
}
/// Iterates through the "component_array" and transforms the values of each pixel back to their floating 
/// point RGB values. Returns an "array2" struct of these RGB values for each pixel.
///
/// # Arguments:
/// * `component_array`: The "array2" of component video values.
pub fn component_to_rgb(component_array: Array2<ImageVid>) -> Array2<ImageRgb> {
    let mut rgb_array = vec![];
    for i in component_array.iter_row_major() {
        let curr_vid_pixel = i.2;
        let y = curr_vid_pixel.y;
        let pb = curr_vid_pixel.pb;
        let pr = curr_vid_pixel.pr;
        let r = ((1.0 * y) + (0.0 * pb) + (1.402 * pr)) * 255.0;
        let g = ((1.0 * y) - (0.344136 * pb) - (0.714136 * pr)) * 255.0;
        let b = ((1.0 * y) + (1.772 * pb) + (0.0 * pr)) * 255.0;
        let curr_rgb_pixel = ImageRgb {
            r:r,
            g:g,
            b:b,
        };
        rgb_array.push(curr_rgb_pixel);
    }

    let rgb_array2 = array2::Array2::from_row_major(component_array.width(), component_array.height(), rgb_array).unwrap();
    return rgb_array2;
}
/// Iterates through the "rgb_array" and transforms the values of each RGB pixel back to 
/// integers. Returns our decompressed image.
///
/// # Arguments:
/// * `rgb_array`: The "array2" of floating point RGB values.
pub fn rgb_to_image(rgb_array: Array2<ImageRgb>) -> Image {
    let mut pixel_array: Vec<Pixel> = vec![];
    for i in rgb_array.iter_row_major() {
        let curr_rgb_pixel = i.2;
        let r = curr_rgb_pixel.r;
        let g = curr_rgb_pixel.g;
        let b = curr_rgb_pixel.b;
        let curr_pixel = Pixel::Rgb(Rgb {
            red:r as u16,
            green:g as u16,
            blue:b as u16,
        });
        pixel_array.push(curr_pixel);


    }
    let decompressed_image = Image {
        pixels: pixel_array, 
        width: rgb_array.width() as u32,
        height: rgb_array.height() as u32,
        denominator: 255,
    };
    return decompressed_image; 
}
/// Goes through and extracts each "ImageCos" value from the 32 bit word and then returns
/// them as a struct of those "ImageCos" values
///
/// # Arguments:
/// * `word`: a 64 bit word containing our "ImageCos" values in the lower 32 bits.
fn unpack_word(word: u64) -> ImageCos{
    let unpack_a = bitpack::getu(word, 9, 23);
    let unpack_b = bitpack::gets(word, 5, 18);
    let unpack_c = bitpack::gets(word, 5, 13);
    let unpack_d = bitpack::gets(word, 5, 8);
    let unpack_pb = bitpack::getu(word, 4, 4);
    let unpack_pr = bitpack::getu(word, 4, 0);
    let current_block = ImageCos{
        a: unpack_a as f32,
        b: unpack_b as f32,
        c: unpack_c as f32,
        d: unpack_d as f32,
        indexed_pb: unpack_pb as usize,
        indexed_pr: unpack_pr as usize,
    };
    return current_block;
}