use array2::Array2;
use crate::transform;

pub fn compress(filename: &str) {
    let base_array = transform::into_array(filename);
    println!("{},{}",base_array.height(),base_array.width());
    let float_array = transform::to_float(base_array);
    let vid_array = transform::rgb_to_comp(float_array);
    let last_array = transform::block_iteration(vid_array);
    

}

pub fn decompress(filename: &str) {
    todo!();
}



