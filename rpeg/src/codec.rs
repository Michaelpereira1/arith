use crate::transform;
use std::env;
#[macro_use] use scan_fmt::scan_fmt;

pub fn compress(filename: &str, output: &str) {
    let (base_array, denom) = transform::into_array(filename);
    //println!("{:?}", base_array);
    let float_array = transform::to_float(base_array, denom);
    //println!("{:?}",float_array);
    let vid_array = transform::rgb_to_comp(float_array);
    let last_array = transform::block_iteration(vid_array);
    println!("{:?}",last_array);
    let new_vid_array = transform::reverse_block(last_array);
    println!("{:#?}", new_vid_array);
    let new_rgb_array = transform::component_to_rgb(new_vid_array);
    println!("{:#?}",new_rgb_array);
    let decompressed_image= transform::rgb_to_image(new_rgb_array);
    decompressed_image.write(Some(output)).unwrap();
    
}

pub fn decompress(filename: &str) {

}




