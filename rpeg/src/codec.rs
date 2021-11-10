use crate::transform;

pub fn compress(filename: &str) {
    let (base_array, denom) = transform::into_array(filename);
    let float_array = transform::to_float(base_array, denom);
    let vid_array = transform::rgb_to_comp(float_array);
    let last_array = transform::block_iteration(vid_array);
    println!("{:?}",last_array);

    

}

pub fn decompress(filename: &str) {
    todo!();
}



