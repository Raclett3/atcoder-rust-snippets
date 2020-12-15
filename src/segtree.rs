use cargo_snippet::snippet;

#[snippet("segtree")]
use std::marker::PhantomData;

#[snippet("segtree")]
trait Identity {
    fn zero() -> Self;
    fn one() -> Self;
    fn top() -> Self;
    fn bottom() -> Self;
}

#[snippet("segtree")]
macro_rules! identity {
    ($($t:ident),+) => {
        $(impl Identity for $t {
            fn zero() -> $t {
                0
            }

            fn one() -> $t {
                1
            }

            fn top() -> $t {
                std::$t::MAX
            }

            fn bottom() -> $t {
                std::$t::MIN
            }
        })*
    };
}

#[snippet("segtree")]
identity!(isize, usize, i8, i16, i32, i64, i128, u8, u16, u32, u64, u128);

#[snippet("segtree")]
pub trait Monoid {
    type S: Clone;
    fn identity() -> Self::S;
    fn product(lhs: &Self::S, rhs: &Self::S) -> Self::S;
}

#[snippet("segtree")]
struct Add<T: std::ops::Add<Output = T> + Clone + Identity>(PhantomData<T>);

#[snippet("segtree")]
impl<T: std::ops::Add<Output = T> + Copy + Identity> Monoid for Add<T> {
    type S = T;

    fn identity() -> Self::S {
        T::zero()
    }

    fn product(&lhs: &Self::S, &rhs: &Self::S) -> Self::S {
        lhs + rhs
    }
}

#[snippet("segtree")]
struct Product<T: std::ops::Mul<Output = T> + Clone + Identity>(PhantomData<T>);

#[snippet("segtree")]
impl<T: std::ops::Mul<Output = T> + Copy + Identity> Monoid for Product<T> {
    type S = T;

    fn identity() -> Self::S {
        T::one()
    }

    fn product(&lhs: &Self::S, &rhs: &Self::S) -> Self::S {
        lhs * rhs
    }
}

#[snippet("segtree")]
struct Max<T: Ord + Clone + Identity>(PhantomData<T>);

#[snippet("segtree")]
impl<T: Ord + Copy + Identity> Monoid for Max<T> {
    type S = T;

    fn identity() -> Self::S {
        T::bottom()
    }

    fn product(&lhs: &Self::S, &rhs: &Self::S) -> Self::S {
        lhs.max(rhs)
    }
}

#[snippet("segtree")]
struct Min<T: Ord + Clone + Identity>(PhantomData<T>);

#[snippet("segtree")]
impl<T: Ord + Copy + Identity> Monoid for Min<T> {
    type S = T;

    fn identity() -> Self::S {
        T::top()
    }

    fn product(&lhs: &Self::S, &rhs: &Self::S) -> Self::S {
        lhs.min(rhs)
    }
}

#[snippet("segtree")]
fn log2_ceil(n: u32) -> u32 {
    32 - n.leading_zeros()
}

#[snippet("segtree")]
pub struct Segtree<M: Monoid> {
    len: usize,
    padded_len: usize,
    log: usize,
    arr: Vec<M::S>,
}

#[snippet("segtree")]
impl<M: Monoid> From<Vec<M::S>> for Segtree<M> {
    fn from(v: Vec<M::S>) -> Self {
        let len = v.len();
        let log = log2_ceil(len as u32) as usize;
        let padded_len = 1 << log;
        let mut arr = vec![M::identity(); 2 * padded_len];
        arr[padded_len..(padded_len + len)].clone_from_slice(&v);
        let mut tree = Segtree {
            len,
            padded_len,
            log,
            arr,
        };
        for i in (1..padded_len).rev() {
            tree.update(i);
        }
        tree
    }
}

#[snippet("segtree")]
impl<M: Monoid> Segtree<M> {
    pub fn set(&mut self, index: usize, value: M::S) {
        assert!(index < self.len);
        let index = index + self.padded_len;
        self.arr[index] = value;
        for i in 1..=self.log {
            self.update(index >> i);
        }
    }

    pub fn get(&mut self, index: usize) -> M::S {
        assert!(index < self.len);
        self.arr[index + self.padded_len].clone()
    }

    pub fn product(&self, l: usize, r: usize) -> M::S {
        assert!(l <= r && r <= self.len);
        let mut l_prod = M::identity();
        let mut r_prod = M::identity();
        let mut l = l + self.padded_len;
        let mut r = r + self.padded_len;

        while l < r {
            if l & 1 != 0 {
                l_prod = M::product(&l_prod, &self.arr[l]);
                l += 1;
            }

            if r & 1 != 0 {
                r -= 1;
                r_prod = M::product(&self.arr[r], &r_prod);
            }
            l >>= 1;
            r >>= 1;
        }

        M::product(&l_prod, &r_prod)
    }

    pub fn product_all(&self) -> M::S {
        self.arr[1].clone()
    }

    pub fn new(n: usize) -> Segtree<M> {
        vec![M::identity(); n].into()
    }

    fn update(&mut self, k: usize) {
        self.arr[k] = M::product(&self.arr[2 * k], &self.arr[2 * k + 1]);
    }
}

#[test]
fn test_segtree() {
    let mut segtree = Segtree::<Add<usize>>::from(vec![10, 20, 30, 40, 50]);
    assert_eq!(segtree.product_all(), 150);
    assert_eq!(segtree.product(0, 3), 60);
    assert_eq!(segtree.product(2, 5), 120);
    assert_eq!(segtree.get(2), 30);
    segtree.set(2, 100);
    assert_eq!(segtree.product_all(), 220);
    assert_eq!(segtree.product(0, 3), 130);
    assert_eq!(segtree.product(2, 5), 190);
    assert_eq!(segtree.get(2), 100);
    let mut segtree = Segtree::<Add<usize>>::new(5);
    segtree.set(0, 10);
    segtree.set(4, 50);
    segtree.set(1, 20);
    segtree.set(3, 40);
    segtree.set(2, 30);
    assert_eq!(segtree.product_all(), 150);
    assert_eq!(segtree.product(0, 3), 60);
    assert_eq!(segtree.product(2, 5), 120);
}
