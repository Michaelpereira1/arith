use std::process::exit;
use array2::Array2;
use csc411_image::Pixel;

struct Image_rgb {
    R: usize,
    G: usize,
    B: usize,
    denom: usize,
}

struct Image_vid {
    Y: usize,
    Pb: usize,
    Pr: usize,
}

struct Image_2x2 {
    Pb_avg: usize,
    Pr_avg: usize,
    Y: usize,
}

struct Image_cos {
    Pb_avg: usize,
    Pr_avg: usize,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
}
    
pub fn into_array(filename: &str) -> Array2<csc411_image::Pixel> {
    let img = csc411_image::Image::read(Some(filename)).unwrap();
    let mut ppm_array2 = array2::Array2::from_row_major(img.width as usize, img.height as usize, img.pixels).unwrap();
    ppm_array2.trim();
    return ppm_array2;
}

pub fn to_float(pixel_array: Array2<Pixel>) {
    for i in pixel_array.iter_row_major() {
        let curr = &i.2;
        match curr {
            Pixel::Gray(Gray) => {
                exit(0);
            }

            Pixel::Rgb(rgb) => {
                
            }
            
        }
    }
    
}





