use crate::comp_transform;
use crate::decomp_transform;

/// Calls the functions in "comp_transform.rs" in order to compress the image given
/// in the argument. Returns our decompressed image in the form of a sequence of 32 bit packed words.
///
/// # Arguments:
/// * `filename`: The file containing the ppm image
pub fn compress(filename: &str) {
    let (base_array, denom) = comp_transform::into_array(filename);
    let float_array = comp_transform::to_float(base_array, denom);
    let vid_array = comp_transform::rgb_to_comp(float_array);
    let last_array = comp_transform::block_iteration(vid_array);
    let word_array = comp_transform::convert_to_words(last_array);
    comp_transform::output_compressed(word_array);
}
/// Calls the functions in "decomp_transform.rs" in order to decompress the image given
/// in the argument. Writes the decompressed image to a file.
///
/// # Arguments:
/// * `filename`: The file containing the compressed ppm image
pub fn decompress(filename: &str) {
    let word_array = decomp_transform::get_compressed(filename);
    let cos_array = decomp_transform::word_to_cos(word_array);
    let vid_array = decomp_transform::reverse_block(cos_array);
    let rgb_array = decomp_transform::component_to_rgb(vid_array);
    let image = decomp_transform::rgb_to_image(rgb_array);
    image.write(None).unwrap();
    
}




