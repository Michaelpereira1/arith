use array2::Array2;
use csc411_image;

pub fn compress(filename: &str) {
    //trim(filename);
    todo!();
}

pub fn decompress(filename: &str) {
    todo!();
}

pub fn into_array(filename: &str){
    let img = csc411_image::Image::read(Some(filename)).unwrap();
    let ppm_array2 = Array2::from_row_major(img.width as usize, img.height as usize, img.pixels).unwrap();
}

pub fn trim(){
    todo!();
}
