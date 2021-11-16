
use std::io::{self, Read, Write};
use std::{process::exit, vec};
use array2::Array2;
use csc411_arith::{chroma_of_index, index_of_chroma};
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
    
pub fn into_array(filename: &str) -> (Array2<csc411_image::Pixel>, u16) {
    let img = csc411_image::Image::read(Some(filename)).unwrap();
    let mut ppm_array2 = array2::Array2::from_row_major(img.width as usize, img.height as usize, img.pixels).unwrap();
    ppm_array2.trim();
    let denom = img.denominator;
    return (ppm_array2,denom);
}

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
            b = b.round();
            
            if c < -0.3 {
                c = -0.3;
            } else if c > 0.3 {
                c = 0.3;
            }
            c = c * 50.0;
            c = c.round();
            
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

pub fn convert_to_output (array_2x2: Array2<ImageCos>) -> Array2<u64> {
    let mut word_vec:Vec<u64> = vec![];
    for i in array_2x2.iter_row_major() {
        let current_block = *i.2;
        let word:u64 = 0;
        let new_word_a = bitpack::newu(word, 9, 23, current_block.a as u64).unwrap();
        let new_word_b = bitpack::news(word, 5, 18, current_block.b as i64).unwrap();
        let new_word_c = bitpack::news(word, 5, 13, current_block.c as i64).unwrap();
        let new_word_d = bitpack::news(word, 5, 8, current_block.d as i64).unwrap();
        let new_word_pb = bitpack::newu(word, 4, 4, current_block.indexed_pb as u64).unwrap();
        let new_word_pr = bitpack::newu(word, 4, 0, current_block.indexed_pr as u64).unwrap();
        let packed_word = new_word_a + new_word_b + new_word_c + new_word_d + new_word_pb + new_word_pr;
        word_vec.push(packed_word);  
    }
    let word_array2 = array2::Array2::from_row_major(array_2x2.width(),array_2x2.height(), word_vec).unwrap();
    return word_array2;
}

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

pub fn unpack_word(word: u64) -> ImageCos{
    //let word:u64 = 2124398510;
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

pub fn output_compressed(word_array: Array2<u64>) {
    println!("Compressed image format 2\n{} {}", (word_array.width() * 2) as u32, (word_array.height() * 2) as u32);
    for i in word_array.iter_row_major() {
        let current_word: u32 = *i.2 as u32;
        let bytes = current_word.to_be_bytes();
        io::stdout().write_all(&bytes).unwrap();
    }
    
}

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