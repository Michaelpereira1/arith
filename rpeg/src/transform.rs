use array2::Array2;

struct Image_rgb {
    R: usize,
    G: usize,
    B: usize,
    denom: usize,
}

struct Image_vid {
    Y: usize,
    Pb: usize,
    Pr: usize,
}

struct Image_2x2 {
    Pb_avg: usize,
    Pr_avg: usize,
    Y: usize,
}

struct Image_cos {
    Pb_avg: usize,
    Pr_avg: usize,
    a: usize,
    b: usize,
    c: usize,
    d: usize,
}

fn get_RGB (pixel_array: Array2<image::pixels) -> Array2<Image_vid> {
    
}