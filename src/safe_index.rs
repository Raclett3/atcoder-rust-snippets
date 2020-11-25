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
    fn at_mut_d(&mut self, index: isize) -> &mut T {
        self.at_mut(index).unwrap()
    }
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

#[test]
fn test_safe_index() {
    let mut v = vec![10, 20, 30, 40, 50];
    assert_eq!(v.at(0), Some(&10));
    assert_eq!(v.at(-2), None);
    assert_eq!(v.at(5), None);
    assert_eq!(v.at(4), Some(&50));
    assert_eq!(v.at_mut(-1), None);
    assert_eq!(v.at_mut(5), None);
    *v.at_mut(2).unwrap() = 60;
    assert_eq!(v.at(2), Some(&60));
}
