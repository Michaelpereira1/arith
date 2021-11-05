pub enum TypeOfIteration {
    RowMajor,
    ColumnMajor
}
#[derive(Clone)]
pub struct Array2<T> {
    pub v: Vec<T>,
    pub width:usize,
    pub height: usize,
}

impl<T> Default for Array2<T>{
    fn default() -> Self {
        Self::new()
    }
}

pub struct Array2Iter<'a, T>{
    pub v: &'a [T],
    pub col: usize,
    pub row: usize,
    pub dim:usize,
    pub type_of_iter:TypeOfIteration
}

impl <'a, T> Array2Iter<'a,T> {
    pub fn new(v: &'a [T], d:usize, toi:TypeOfIteration) -> Self {
        Self {v, col:0, dim:d, row: 0, type_of_iter: toi} 
    }
}

impl<T> Array2<T> {
    pub fn new() -> Self {
        Array2 {
            v: Vec::<T>::new(),
            width: 0,
            height: 0,
        }
    }

    pub fn new_h_w(vec: Vec<T>, dim: usize) -> Self { 
        Array2 {
            v: vec,
            width: dim,
            height: dim
        }
    }

    pub fn at(&self, x:usize, y:usize) ->  Option<&T> {
        let loction = x * self.height + y;
        if loction < self.v.len() {
            return Some(&self.v[loction]);
        }
        else{
            None
        }
    }

    pub fn make_iter(&self, toi:TypeOfIteration) -> Array2Iter<T>{
        return Array2Iter::new(&self.v, self.height, toi);
    }
}

impl<'a, T> Iterator for Array2Iter<'a, T> {
    type Item = &'a T;
    
    fn next(&mut self) -> Option<Self::Item> {
        match self.type_of_iter {
            TypeOfIteration::RowMajor => {
                if self.col < self.v.len(){
                    self.row = self.col;
                    self.col += 1;
                    return Some(&self.v[self.row]);
                }
                else{
                    return None
                }
            }
            TypeOfIteration::ColumnMajor => {
                if self.row >= self.v.len()/self.dim{
                    self.col += 1;
                    self.row = 0;
                }
                if self.col < self.dim{
                    let r = &self.v[self.row * self.dim + self.col];
                    self.row += 1;
                    return Some(r);
                }
                else{
                    return None
                }
            }
        }
    }
}  

#[cfg(test)]
mod tests {
    use crate::Array2;

    #[test]
    fn test_default_construction() {
        let a2 = Array2::<String>::new();
        assert_eq!(a2.height, 0);
        assert_eq!(a2.width,0);
    }
    #[test]
    fn test_udef_construction() {
        let a2 = Array2::<i32>::new_h_w(vec![1,2,3], 3);
        assert_eq!(a2.width,3);
        assert_eq!(a2.height,3);    
    }
    #[test]
    fn test_array_bounds() {
        let a2 = Array2::<i32>::new_h_w(vec![1,2,3,4,5,6], 3);
        assert_eq!(a2.at(0, 1), Some(&2));
        assert_eq!(a2.at(1,0), Some(&4));
        assert_eq!(a2.at(2,0), None);
    }
    
    #[test]
    fn test_array_iteration() {
        let a2 = Array2::<i32>::new_h_w(vec![1,2,3,4,5,6,7,8,9], 3);
        let sum = a2.make_iter(crate::TypeOfIteration::RowMajor).fold(0, |acc, x| acc + x);
        let sum2 = a2.make_iter(crate::TypeOfIteration::ColumnMajor).fold(1, |acc, x| acc + x);
        assert_eq!(sum, 45);
        assert_eq!(sum2, 46);
    }
}
