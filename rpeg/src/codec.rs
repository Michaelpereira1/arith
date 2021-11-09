use array2::Array2;
use crate::transform;

pub fn compress(filename: &str) {
    let base_array = transform::into_array(filename);
    let float_array = transform::to_float(base_array);
    let vid_array = transform::rgb_to_comp(float_array);
    println!("{:?}", vid_array)
    

}

pub fn decompress(filename: &str) {
    todo!();
}



