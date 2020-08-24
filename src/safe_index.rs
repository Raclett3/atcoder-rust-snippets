use cargo_snippet::snippet;

#[snippet("index")]
pub trait SafeIndex<T> {
    fn at(&self, index: isize) -> Option<&T>;
    fn at_mut(&mut self, index: isize) -> Option<&mut T>;
}

#[snippet("index")]
impl<T> SafeIndex<T> for std::vec::Vec<T> {
    fn at(&self, index: isize) -> Option<&T> {
        let length = self.len() as isize;
        if 0 <= index && index < length {
            Some(&self[index as usize])
        } else {
            None
        }
    }

    fn at_mut(&mut self, index: isize) -> Option<&mut T> {
        let length = self.len() as isize;
        if 0 <= index && index < length {
            Some(&mut self[index as usize])
        } else {
            None
        }
    }
}
