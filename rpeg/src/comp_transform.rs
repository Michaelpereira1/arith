use std::io::{self,Write};
use std::{process::exit, vec};
use array2::Array2;
use csc411_arith::index_of_chroma;
use csc411_image::Pixel;
use csc411_image;
use bitpack::bitpack;

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

/// Loads the ppm image from a file, trimming the width/height if necessary.
/// Returns a "array2" struct of pixels with our RGB values. 
///
/// # Arguments:
/// * `filename`: The file containing the ppm image
pub fn into_array(filename: &str) -> (Array2<csc411_image::Pixel>, u16) {
    let img = csc411_image::Image::read(Some(filename)).unwrap();
    let mut ppm_array2 = array2::Array2::from_row_major(img.width as usize, img.height as usize, img.pixels).unwrap();
    ppm_array2.trim();
    let denom = img.denominator;
    return (ppm_array2,denom);
}

/// Iterates through our "array2" of pixels and transforms each value into a
/// floating point representation. Returns an "array2" struct of pixels with the RGB values being 
/// represented by floating point numbers.
///
/// # Arguments:
/// * `pixel_array`: The "array2" of pixels
/// * `denom`: The denominator of the RGB values
pub fn to_float(pixel_array: Array2<Pixel>, denom: u16) -> Array2<ImageRgb> {
    let mut float_array: Vec<ImageRgb> = vec![];
    for i in pixel_array.iter_row_major() {
        let curr = i.2;
        match curr {
            Pixel::Gray(_gray) => {
                exit(0);
            }
            Pixel::Rgb(rgb) => {
                let cur_red= rgb.red as f32 / denom as f32;
                let cur_blue:f32 = rgb.blue as f32 / denom as f32;
                let cur_green:f32 = rgb.green as f32 / denom as f32;
                let cur_pixel = ImageRgb {
                    r:cur_red, 
                    b:cur_blue, 
                    g:cur_green
                };
                float_array.push(cur_pixel);
            }
        }
    }
    let float_array2 = array2::Array2::from_row_major(pixel_array.width(), pixel_array.height(), float_array).unwrap();
    return float_array2;
}
/// Iterates through our "array2" of pixels and transforms the RGB values to 
/// component video values. Returns an "array2" struct of pixels with component video values.
///
/// # Arguments:
/// * `rgb_array`: The "array2" of pixels as f32's
pub fn rgb_to_comp(rgb_array: Array2<ImageRgb>) -> Array2<ImageVid> {
    let mut temp_array: Vec<ImageVid> = vec![];
    fn rgb_conv (rgb: ImageRgb) -> ImageVid {
        let y = (0.299 * rgb.r) + (0.587 * rgb.g) + (0.114 * rgb.b);
        let pb = (-0.168736 * rgb.r) - (0.331264 * rgb.g) + (0.5 * rgb.b);
        let pr = (0.5 * rgb.r) - (0.418688 * rgb.g) - (0.081312 * rgb.b);
        return ImageVid { y: y, pb: pb, pr: pr}
    }
    for i in rgb_array.iter_row_major() {
        temp_array.push(rgb_conv(*i.2))
    }
    let comp_array2 = array2::Array2::from_row_major(rgb_array.width(),rgb_array.height(), temp_array).unwrap();
    return comp_array2;

}
/// Iterates through our "array2" of component video values in 2x2 blocks in order to 
/// perform DCT and it returns an "array2" struct of the values we later need to pack into a word.
/// These values are a, b, c, d, average pb, and average pr.
///
/// # Arguments:
/// * `vid_array`: The "array2" of component video values
pub fn block_iteration(vid_array: Array2<ImageVid>) -> Array2<ImageCos> {
    let mut transformed_2x2: Vec<ImageCos> = vec![];
    for j in (0..vid_array.height()).step_by(2) {
        for i in (0..vid_array.width()).step_by(2) {
            let zero_zero = vid_array.get(i,j).unwrap();
            let zero_one = vid_array.get(i+1,j).unwrap();
            let one_zero = vid_array.get(i,j+1).unwrap();
            let one_one = vid_array.get(i+1,j+1).unwrap();
            let avg_pb = (zero_zero.pb + zero_one.pb + one_one.pb + one_zero.pb) / 4.0;
            let avg_pr = (zero_zero.pr + zero_one.pr + one_one.pr + one_zero.pr) / 4.0;
            let index_pb = index_of_chroma(avg_pb);
            let index_pr = index_of_chroma(avg_pr);
            let mut a = (one_one.y + one_zero.y + zero_one.y + zero_zero.y) / 4.0;
            let mut b = (one_one.y + one_zero.y - zero_one.y - zero_zero.y) /4.0;
            let mut c = (one_one.y - one_zero.y + zero_one.y - zero_zero.y) / 4.0;
            let mut d = (one_one.y - one_zero.y - zero_one.y + zero_zero.y) /4.0;
            if a < 0.0 {
                a = 0.0;
            } else if a > 1.0 {
                a = 1.0;
            }

            a = a * 511.0;
            a = a.floor();
            
            if b < -0.3 {
                b = -0.3;
            } else if b > 0.3 {
                b = 0.3;
            }
            b = b * 50.0;
            b = b.floor();
            
            if c < -0.3 {
                c = -0.3;
            } else if c > 0.3 {
                c = 0.3;
            }
            c = c * 50.0;
            c = c.floor();
            
            if d < -0.3 {
                d = -0.3;
            } else if d > 0.3 {
                d = 0.3;
            }
            d = d * 50.0;
            d = d.floor();
            if d > 15.0 {
                d = 15.0;
            }

            let curr_block = ImageCos {
                indexed_pb:index_pb,
                indexed_pr:index_pr,
                a:a as f32,
                b:b as f32,
                c:c as f32,
                d:d as f32,
            };
            transformed_2x2.push(curr_block);
        }
        
    }
    let transformed_2x2_array = array2::Array2::from_row_major(vid_array.width() / 2, vid_array.height() / 2, transformed_2x2).unwrap();
    return transformed_2x2_array;

}
/// Iterates through our "array2" of values that need to be packed into a word and packs
/// each value of the "array2" ImageCos struct and packs it into the lower 32 bits of a 64 bit word. 
/// returns an "array2" of these words.
///
/// # Arguments:
/// * `array_2x2`: The "array2" of a, b, c, d, avg pb, and avg pr
pub fn convert_to_words (array_2x2: Array2<ImageCos>) -> Array2<u64> {
    let mut word_vec:Vec<u64> = vec![];
    for i in array_2x2.iter_row_major() {
        let current_block = *i.2;
        let current_word = pack_word(current_block);
        word_vec.push(current_word);  
    }
    let word_array2 = array2::Array2::from_row_major(array_2x2.width(),array_2x2.height(), word_vec).unwrap();
    return word_array2;
}

/// Takes each word in the "word_array" and writes them to disk. 
///
/// # Arguments:
/// * `word_array`: the "array2" of 32 bit words.  
pub fn output_compressed(word_array: Array2<u64>) {
    println!("Compressed image format 2\n{} {}", (word_array.width() * 2) as u32, (word_array.height() * 2) as u32);
    for i in word_array.iter_row_major() {
        let current_word: u32 = *i.2 as u32;
        let bytes = current_word.to_be_bytes();
        io::stdout().write_all(&bytes).unwrap();
    }
    
}
/// Takes an "empty" 64 bit word of all zeroes and packs each value of the ImageCos struct into it's
/// proper position within the lower 32 bits of the word, and then returns our newly packed word. 
///
/// # Arguments:
/// * `block`: a single element of our "array2" of ImageCos. Contains the values we need to pack into a word. 
fn pack_word(block: ImageCos) -> u64 {
    let word:u64 = 0;
        let new_word_a = bitpack::newu(word, 9, 23, block.a as u64).unwrap();
        let new_word_b = bitpack::news(word, 5, 18, block.b as i64).unwrap();
        let new_word_c = bitpack::news(word, 5, 13, block.c as i64).unwrap();
        let new_word_d = bitpack::news(word, 5, 8, block.d as i64).unwrap();
        let new_word_pb = bitpack::newu(word, 4, 4, block.indexed_pb as u64).unwrap();
        let new_word_pr = bitpack::newu(word, 4, 0, block.indexed_pr as u64).unwrap();
        let packed_word = new_word_a + new_word_b + new_word_c + new_word_d + new_word_pb + new_word_pr;
        return packed_word;
}