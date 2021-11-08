// `Array2` provides a fixed-size 2-dimensional array.
// An error that can arise during the use of an [`Array2D`].
//
// [`Array2D`]: struct.Array2D.html
// #[derive(Debug, Eq, PartialEq)]
// pub enum Error {
//     /// The indices (coordinates) were out of bounds.
//     IndicesOutOfBounds(usize, usize),
// }

/// Elements contained must support `Clone`
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct Array2<T: Clone> {
    width: usize,
    height: usize,
    data: Vec<T>,
}

impl<T: Clone> Array2<T> {
    /// Creates a new `Array2`.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`.
    /// * `height`: the height of the `Array2`.
    /// * `val`: the value to fill every element with
    pub fn new(width: usize, height: usize, val: T) -> Self {
        let data = vec![val; width * height];
        Array2 {
            width,
            height,
            data,
        }
    }

    /// Creates a new `Array2` from a Vec<T>.
    ///
    /// # Arguments
    ///
    /// * `width`: the width of the `Array2`
    /// * `height`: the height of the `Array2`
    /// * `values`: A Vec<T>, in row-major order, whose
    ///             length must be `width` * `height`.
    pub fn from_row_major(width: usize, height: usize, values: Vec<T>) -> Result<Self, String> {
        if width * height != values.len() {
            Err(format!(
                "Values has {} elements, which is not the product of width {} and height {}",
                values.len(),
                width,
                height,
            ))
        } else {
            Ok(Array2 {
                width,
                height,
                data: values,
            })
        }
    }

    /// The height
    pub fn height(&self) -> usize {
        self.height
    }

    /// The width
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns a reference to the element at the given `column` and `row`
    /// as long as that index is in bounds
    /// (wrapped in [`Some`]). Returns [`None`] if out of bounds.
    pub fn get(&self, c: usize, r: usize) -> Option<&T> {
        self.get_index(c, r).map(|index| &self.data[index])
    }

    pub fn get_mut(&mut self, c: usize, r: usize) -> Option<&mut T> {
        self.get_index(c, r).map( move |index| &mut self.data[index])
    }

    fn get_index(&self, c: usize, r: usize) -> Option<usize> {
        if c < self.width && r < self.height {
            Some(r * self.width + c)
        } else {
            None
        }
    }

    pub fn remove(&mut self, c: usize, r: usize) {
        self.get_index(c, r).map( move |index|  self.data.remove(index));

    }

    pub fn trim(&mut self){        
        if self.width % 2 != 0 {
            //trim the farthest right column from the self.        
            for i in (0..self.height()).rev() {
                self.remove(self.height()-1, i);
            }        
            self.width -= 1; 
            
        }
        if self.height() % 2 != 0 {
            //trim the bottom row from the self.
            for i in (0..self.width).rev(){
                self.remove(i, self.width);
            }       
        }
    }

    pub fn iter_row_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        // The compiler knows to optimize away the div-mod ops.
        self.data
            .iter()
            .enumerate()
            .map(move |(i, v)| (i % self.width, i / self.width, v))
    }

    pub fn iter_col_major(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        (0..self.width)
            // get the start of every column as a fresh iter and keep the index of the column
            // skip advances the iterator without yielding items
            .map(move |c| (c, self.data.iter().skip(c)))
            // do a flat_map for all the columns
            .flat_map(move |(c, col)| {
                // for each iterator on a column, step forward by width for the correct next element in that column
                // step_by yields an item and then advances the iterator
                col.step_by(self.width)
                    // enumerate down the columns to get the index of the row
                    .enumerate()
                    .map(move |(r, val)| (c, r, val))
            })

        // (0..self.width)
        //    .flat_map(move |c| (0..self.height)
        //    .map(move |r| (c, r, self.get(c, r).unwrap())))
    }
}

#[cfg(test)]
mod tests { 
    use crate::Array2;  

    #[test]
    fn test_array() {
        let mut a2 = Array2::from_row_major(3, 3, vec![1,2,3,4,5,6,7,8,9]).unwrap();
        a2.trim();        
        assert_eq!(a2.data, [1,2,4,5])
    }
}