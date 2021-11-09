use std::{arch::x86_64::_mm256_undefined_si256, process::exit};
use array2::Array2;
use csc411_arith::index_of_chroma;
use csc411_image::Pixel;

#[derive(Debug,Clone,Copy)]
pub struct Image_rgb {
    R: f32,
    G: f32,
    B: f32,
}

#[derive(Debug,Clone)]
pub struct Image_vid {
    Y: f32,
    Pb: f32,
    Pr: f32,
}

#[derive(Debug,Clone)]
pub struct Image_cos {
    Pb_avg: f32,
    Pr_avg: f32,
    A: f32,
    B: f32,
    C: f32,
    D: f32,
}
    
pub fn into_array(filename: &str) -> Array2<csc411_image::Pixel> {
    let img = csc411_image::Image::read(Some(filename)).unwrap();
    let mut ppm_array2 = array2::Array2::from_row_major(img.width as usize, img.height as usize, img.pixels).unwrap();
    ppm_array2.trim();
    return ppm_array2;
}

pub fn to_float(pixel_array: Array2<Pixel>) -> Array2<Image_rgb> {
    let mut float_array: Vec<Image_rgb> = vec![];
    for i in pixel_array.iter_row_major() {
        let curr = &i.2;
        match curr {
            Pixel::Gray(Gray) => {
                exit(0);
            }

            Pixel::Rgb(rgb) => {
                let cur_red= (rgb.red as f32/ 255.0);
                let cur_blue:f32 = (rgb.blue as f32 / 255.0);
                let cur_green:f32 = (rgb.green as f32 / 255.0);
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
        return Image_vid { Y: y, Pb: pb, Pr: pr }
    }
    for i in rgb_array.iter_row_major() {
        temp_array.push(rgb_conv(*i.2))
    }
    let comp_array2 = array2::Array2::from_row_major(rgb_array.width(),rgb_array.height(), temp_array).unwrap();
    return comp_array2;

}

pub fn block_iteration(vid_array: Array2<Image_vid>) -> Array2<Image_cos> {
    let mut transformed_2x2: Vec<Image_cos> = vec![];
    println!("width-{},height-{}", vid_array.width(), vid_array.height());
    for i in (0..vid_array.height()).step_by(2) {
        for j in (0..vid_array.width()).step_by(2) {
            let zero_zero = vid_array.get(j,i).unwrap();
            let zero_one = vid_array.get(j,i+1).unwrap();
            let one_zero = vid_array.get(j+1,i).unwrap();
            let one_one = vid_array.get(j+1,i+1).unwrap();
            let avg_pb = (zero_zero.Pb + zero_one.Pb + one_one.Pb + one_zero.Pb) / 4.0;
            let avg_pr = (zero_zero.Pr + zero_one.Pr + one_one.Pr + one_zero.Pr) / 4.0;
            let index_pb = index_of_chroma(avg_pb);
            let index_pr = index_of_chroma(avg_pr);
            let mut a = (one_one.Y + zero_one.Y + one_zero.Y + zero_zero.Y) / 4.0;
            let mut b = (one_one.Y + one_zero.Y - zero_one.Y - zero_zero.Y) /4.0;
            let mut c = (one_one.Y - one_zero.Y + zero_one.Y - zero_zero.Y) / 4.0;
            let mut d = (one_one.Y - one_zero.Y - zero_one.Y + zero_zero.Y) /4.0;

            let a = a * 511.0;
            a.round();

            b = b * 50.0;
            b.round();
            if b > 15.0 {
                b = 15.0;
            }
            c = c * 50.0;
            c.round();
            if c > 15.0 {
                c = 15.0;
            }
            d = d * 50.0;
            d.round();
            if d > 15.0 {
                d = 15.0;
            }

            let mut curr_block = Image_cos {
                Pb_avg:avg_pb,
                Pr_avg:avg_pr,
                A:a,
                B:b,
                C:c,
                D:d,
            };
            transformed_2x2.push(curr_block);
        }
        
    }
    let transformed_2x2_array = array2::Array2::from_row_major(vid_array.width() / 2, vid_array.height() / 2, transformed_2x2).unwrap();
    return transformed_2x2_array;

}






