use std::process::exit;
use array2::Array2;
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

struct Image_2x2 {
    Pb_avg: f32,
    Pr_avg: f32,
    Y: f32,
}

struct Image_cos {
    Pb_avg: f32,
    Pr_avg: f32,
    a: f32,
    b: f32,
    c: f32,
    d: f32,
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







