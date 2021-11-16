

pub fn convert_to_words (array_2x2: Array2<ImageCos>) -> Array2<u64> {
    let mut word_vec:Vec<u64> = vec![];
    for i in array_2x2.iter_row_major() {
        let current_block = *i.2;
        let current_word = pack_word(current_block);
        word_vec.push(current_word);  
    }
    let word_array2 = array2::Array2::from_row_major(array_2x2.width(),array_2x2.height(), word_vec).unwrap();
    return word_array2;
}







pub fn output_compressed(word_array: Array2<u64>) {
    println!("Compressed image format 2\n{} {}", (word_array.width() * 2) as u32, (word_array.height() * 2) as u32);
    for i in word_array.iter_row_major() {
        let current_word: u32 = *i.2 as u32;
        let bytes = current_word.to_be_bytes();
        io::stdout().write_all(&bytes).unwrap();
    }
    
}
