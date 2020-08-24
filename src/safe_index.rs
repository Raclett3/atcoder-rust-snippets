use cargo_snippet::snippet;

#[snippet("index")]
pub trait SafeIndex<T> {
    fn at(&self, index: isize) -> Option<&T>;
    fn at_d(&self, index: isize) -> &T {
        self.at(index).unwrap()
    }
}

#[snippet("index")]
impl<T> SafeIndex<T> for Vec<T> {
    fn at(&self, index: isize) -> Option<&T> {
        let length = self.len() as isize;
        if 0 <= index && index < length {
            Some(&self[index as usize])
        } else {
            None
        }
    }
}

#[snippet("index")]
impl<T> SafeIndex<T> for &[T] {
    fn at(&self, index: isize) -> Option<&T> {
        let length = self.len() as isize;
        if 0 <= index && index < length {
            Some(&self[index as usize])
        } else {
            None
        }
    }
}

#[snippet("index")]
impl<T> SafeIndex<T> for &mut [T] {
    fn at(&self, index: isize) -> Option<&T> {
        let length = self.len() as isize;
        if 0 <= index && index < length {
            Some(&self[index as usize])
        } else {
            None
        }
    }
}

#[snippet("indexmut")]
pub trait SafeIndexMut<T> {
    fn at_mut(&mut self, index: isize) -> Option<&mut T>;
}

#[snippet("indexmut")]
impl<T> SafeIndexMut<T> for Vec<T> {
    fn at_mut(&mut self, index: isize) -> Option<&mut T> {
        let length = self.len() as isize;
        if 0 <= index && index < length {
            Some(&mut self[index as usize])
        } else {
            None
        }
    }
}

#[snippet("indexmut")]
impl<T> SafeIndexMut<T> for &mut [T] {
    fn at_mut(&mut self, index: isize) -> Option<&mut T> {
        let length = self.len() as isize;
        if 0 <= index && index < length {
            Some(&mut self[index as usize])
        } else {
            None
        }
    }
}
