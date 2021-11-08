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
    let ppm_array2 = array2::Array2::from_row_major(img.width as usize, img.height as usize, img.pixels).unwrap();
    trim(ppm_array2);
}

pub fn trim<T: Clone>(image: array2::Array2<T>){    
    
    if image.width() % 2 != 0 {
        //trim the farthest right column from the image.        
        for i in (0..image.height()).rev() {
            image.remove(image.height()-1, i);
        }        
        let mut image_trim = Array2::from_row_major(image.width-1, image.height, image.data).unwrap();        
        
    }else if image.height() % 2 != 0 {
        //trim the bottom row from the image.
        for i in (0..Image_trim.width()).rev(){
            Image_trim.remove(i, Image_trim.width());
        }       
    }
}

#[cfg(test)]
mod tests { 
    //use crate::Array2;  

    #[test]
    fn test_array() {
        let mut a2 = Array2::from_row_major(3, 3, vec![1,2,3,4,5,6,7,8,9]).unwrap();
        
        assert_eq!(trim(a2), [1,2,4,5,7,8]);
        assert_eq!(a2_trim.data, [1,2,4,5])
    }
}
