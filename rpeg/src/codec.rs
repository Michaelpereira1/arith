use array2::Array2;
use crate::transform;

pub fn compress(filename: &str) {
    let base_array = transform::into_array(filename);
    transform::to_float(base_array);
    

}

pub fn decompress(filename: &str) {
    todo!();
}



