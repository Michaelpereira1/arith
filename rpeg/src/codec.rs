use array2::array2::Array2;
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

pub fn trim<T: Clone>(image: Array2<T>){
    let image_width = Array2::width(&image);
    let image_height = Array2::height(&image);
    if image_width % 2 != 0 {
        //trim the farthest right column from the image.
        todo!();
    }else if image_height % 2 != 0 {
        //trim the bottom row from the image.
        todo!();
    }
}
