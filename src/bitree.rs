use cargo_snippet::snippet;

#[snippet("bit")]
use std::ops::{Add, AddAssign, SubAssign};

#[snippet("bit")]
struct BIT<T: AddAssign + SubAssign + Copy> {
    bit: Vec<T>,
}

#[snippet("bit")]
impl<T: Add + AddAssign + SubAssign + Default + Copy> BIT<T> {
    fn new(length: usize) -> Self {
        BIT {
            bit: vec![T::default(); length],
        }
    }

    fn add(&mut self, mut idx: usize, val: T) {
        while idx < self.bit.len() {
            self.bit[idx] += val;
            idx |= idx + 1;
        }
    }

    fn sub(&mut self, mut idx: usize, val: T) {
        while idx < self.bit.len() {
            self.bit[idx] -= val;
            idx |= idx + 1;
        }
    }

    fn sum(&mut self, mut idx: usize) -> T {
        let mut sum = T::default();
        loop {
            sum += self.bit[idx];
            if idx & (idx + 1) == 0 {
                break sum;
            }
            idx = (idx & (idx + 1)) - 1;
        }
    }
}

#[test]
fn test_bit() {
    let mut bit: BIT<isize> = BIT::new(10);
    for (i, &val) in [7, 2, 9, 8, 4, 1, 5, 0, 20, 4].iter().enumerate() {
        bit.add(i, val);
    }
    for (i, &val) in [1, 3, 5, 7, 0, 2, 4, 6, 8, 10].iter().enumerate() {
        bit.sub(i, val);
    }
    let expected = [6, 5, 9, 10, 14, 13, 14, 8, 20, 14];
    for (i, &val) in expected.iter().enumerate() {
        assert_eq!(bit.sum(i), val);
    }
}
