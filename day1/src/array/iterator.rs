use super::Array;
pub struct ArrayIterator<'a, T: Array> {
    array: &'a T,
    pos: usize,
}

impl<'a, T: Array> Iterator for ArrayIterator<'a, T> {
    type Item = Option<T::RefItem<'a>>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.pos >= self.array.len() {
            None
        } else {
            let pos = self.pos;
            self.pos += 1;
            Some(self.array.get(pos))
        }
    }
}

impl<'a, T: Array> ArrayIterator<'a, T> {
    pub fn new(array: &'a T) -> Self {
        Self { array, pos: 0 }
    }
}
