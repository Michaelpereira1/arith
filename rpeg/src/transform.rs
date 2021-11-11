use std::{arch::x86_64::_mm256_undefined_si256, process::exit, vec};
use array2::Array2;
use csc411_arith::{chroma_of_index, index_of_chroma};
use csc411_image::{Image, Pixel, Rgb};
use csc411_image;

#[derive(Debug,Clone,Copy)]
pub struct Image_rgb {
    R: f32,
    G: f32,
    B: f32,
}

#[derive(Debug,Clone,Copy)]
pub struct Image_vid {
    Y: f32,
    Pb: f32,
    Pr: f32,
    index: usize,
}

#[derive(Debug,Clone,Copy)]
pub struct Image_cos {
    indexed_pb: usize,
    indexed_pr: usize,
    A: f32,
    B: f32,
    C: f32,
    D: f32,
}
    
pub fn into_array(filename: &str) -> (Array2<csc411_image::Pixel>, u16) {
    let img = csc411_image::Image::read(Some(filename)).unwrap();
    img.write(None).unwrap();
    let mut ppm_array2 = array2::Array2::from_row_major(img.width as usize, img.height as usize, img.pixels).unwrap();
    ppm_array2.trim();
    let denom = img.denominator;
    return (ppm_array2,denom);
}

pub fn to_float(pixel_array: Array2<Pixel>, denom: u16) -> Array2<Image_rgb> {
    let mut float_array: Vec<Image_rgb> = vec![];
    for i in pixel_array.iter_row_major() {
        let curr = i.2;
        match curr {
            Pixel::Gray(Gray) => {
                exit(0);
            }
            Pixel::Rgb(rgb) => {
                let cur_red= rgb.red as f32 / denom as f32;
                let cur_blue:f32 = rgb.blue as f32 / denom as f32;
                let cur_green:f32 = rgb.green as f32 / denom as f32;
                let mut cur_pixel = Image_rgb {
                    R:cur_red, 
                    B:cur_blue, 
                    G:cur_green
                };
                float_array.push(cur_pixel);
            }
        }
    }
    let float_array2 = array2::Array2::from_row_major(pixel_array.width(), pixel_array.height(), float_array).unwrap();
    return float_array2;
}

pub fn rgb_to_comp(rgb_array: Array2<Image_rgb>) -> Array2<Image_vid> {
    let mut temp_array: Vec<Image_vid> = vec![];
    fn rgb_conv (rgb: Image_rgb) -> Image_vid {
        let y = (0.299 * rgb.R) + (0.587 * rgb.G) + (0.114 * rgb.B);
        let pb = (-0.168736 * rgb.R) - (0.331264 * rgb.G) + (0.5 * rgb.B);
        let pr = (0.5 * rgb.R) - (0.418688 * rgb.G) - (0.081312 * rgb.B);
        return Image_vid { Y: y, Pb: pb, Pr: pr, index:0}
    }
    for i in rgb_array.iter_row_major() {
        temp_array.push(rgb_conv(*i.2))
    }
    let comp_array2 = array2::Array2::from_row_major(rgb_array.width(),rgb_array.height(), temp_array).unwrap();
    return comp_array2;

}

pub fn block_iteration(vid_array: Array2<Image_vid>) -> Array2<Image_cos> {
    let mut transformed_2x2: Vec<Image_cos> = vec![];
    for j in (0..vid_array.height()).step_by(2) {
        for i in (0..vid_array.width()).step_by(2) {
            let zero_zero = vid_array.get(i,j).unwrap();
            let zero_one = vid_array.get(i+1,j).unwrap();
            let one_zero = vid_array.get(i,j+1).unwrap();
            let one_one = vid_array.get(i+1,j+1).unwrap();
            let avg_pb = (zero_zero.Pb + zero_one.Pb + one_one.Pb + one_zero.Pb) / 4.0;
            let avg_pr = (zero_zero.Pr + zero_one.Pr + one_one.Pr + one_zero.Pr) / 4.0;
            let index_pb = index_of_chroma(avg_pb);
            let index_pr = index_of_chroma(avg_pr);
            let a = (one_one.Y + one_zero.Y + zero_one.Y + zero_zero.Y) / 4.0;
            let mut b = (one_one.Y + one_zero.Y - zero_one.Y - zero_zero.Y) /4.0;
            let mut c = (one_one.Y - one_zero.Y + zero_one.Y - zero_zero.Y) / 4.0;
            let mut d = (one_one.Y - one_zero.Y - zero_one.Y + zero_zero.Y) /4.0;
            let a = a * 511.0;
            let a = a.round();
            
            b = b * 50.0;
            b = b.round();
            if b > 15.0 {
                b = 15.0;
            }
            
            c = c * 50.0;
            c = c.round();
            if c > 15.0 {
                c = 15.0;
            }
            d = d * 50.0;
            d = d.round();
            if d > 15.0 {
                d = 15.0;
            }

            let curr_block = Image_cos {
                indexed_pb:index_pb,
                indexed_pr:index_pr,
                A:a as f32,
                B:b as f32,
                C:c as f32,
                D:d as f32,
            };
            transformed_2x2.push(curr_block);
        }
        
    }
    let transformed_2x2_array = array2::Array2::from_row_major(vid_array.width() / 2, vid_array.height() / 2, transformed_2x2).unwrap();
    return transformed_2x2_array;

}


pub fn reverse_block(transformed_array: Array2<Image_cos>) -> Array2<Image_vid>{
    let mut counter = 0;
    let width = transformed_array.width() * 2;
    let height = transformed_array.height() * 2;
    let blank_struct = Image_vid {Y: 0.0 , Pb:0.0, Pr: 0.0, index:0};
    let mut working_vec: Vec<Image_vid> = vec![blank_struct; width * height];
    for i in transformed_array.iter_row_major() {
        if counter == 0 {
            counter = 0;
        } else if counter % width == 0 {
            counter += width;
        }
        let curr_block = i.2;
        let a = curr_block.A / 511.0;
        let b = curr_block.B / 50.0;
        let c = curr_block.C / 50.0;
        let d = curr_block.D / 50.0;
        let y1 = a - b - c + d;
        let y2 = a - b + c - d;
        let y3 = a + b - c - d;
        let y4 = a + b + c + d;
        println!("{},{},{},{}", y1,y2,y3,y4);
        let avg_pb = chroma_of_index(curr_block.indexed_pb);
        let avg_pr = chroma_of_index(curr_block.indexed_pr);
        let curr_pixel1 = Image_vid {
            Y: y1,
            Pb: avg_pb,
            Pr: avg_pr,
            index: counter,
        };
        let curr_pixel2 = Image_vid {
            Y: y2,
            Pb: avg_pb,
            Pr: avg_pr,
            index: counter + 1,
            
        };
        let curr_pixel3 = Image_vid {
            Y: y3,
            Pb: avg_pb,
            Pr: avg_pr,
            index: counter + width,
        };
        let curr_pixel4 = Image_vid {
            Y: y4,
            Pb: avg_pb,
            Pr: avg_pr,
            index: counter + width + 1,
            
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

pub fn component_to_rgb(component_array: Array2<Image_vid>) -> Array2<Image_rgb> {
    let mut rgb_array = vec![];
    for i in component_array.iter_row_major() {
        let curr_vid_pixel = i.2;
        let y = curr_vid_pixel.Y;
        let pb = curr_vid_pixel.Pb;
        let pr = curr_vid_pixel.Pr;
        let r = ((1.0 * y) + (0.0 * pb) + (1.402 * pr)) * 100.0;
        let g = ((1.0 * y) - (0.344136 * pb) - (0.714136 * pr)) * 100.0;
        let b = ((1.0 * y) + (1.772 * pb) + (0.0 * pr)) * 100.0;
        let curr_rgb_pixel = Image_rgb {
            R:r,
            G:g,
            B:b,
        };
        rgb_array.push(curr_rgb_pixel);
    }

    let rgb_array2 = array2::Array2::from_row_major(component_array.width(), component_array.height(), rgb_array).unwrap();
    return rgb_array2;
}

pub fn rgb_to_image(rgb_array: Array2<Image_rgb>) -> Image {
    let mut pixel_array: Vec<Pixel> = vec![];
    for i in rgb_array.iter_row_major() {
        let curr_rgb_pixel = i.2;
        let r = curr_rgb_pixel.R;
        let g = curr_rgb_pixel.G;
        let b = curr_rgb_pixel.B;
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


