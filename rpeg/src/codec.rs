use crate::transform::{self, output_compressed};
use std::{array, env, process::Output};

pub fn compress(filename: &str) {
    let (base_array, denom) = transform::into_array(filename);
    let float_array = transform::to_float(base_array, denom);
    let vid_array = transform::rgb_to_comp(float_array);
    let last_array = transform::block_iteration(vid_array);
    let word_array = transform::convert_to_output(last_array);
    transform::output_compressed(word_array);
}

pub fn decompress(filename: &str) {
    let word_array = transform::get_compressed(filename);
    let cos_array = transform::word_to_cos(word_array);
    let vid_array = transform::reverse_block(cos_array);
    let rgb_array = transform::component_to_rgb(vid_array);
    let image = transform::rgb_to_image(rgb_array);
    image.write(None).unwrap();
    
}




