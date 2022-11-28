use cargo_snippet::snippet;

#[snippet("segtree")]
use std::marker::PhantomData;

#[snippet("segtree")]
trait Identity: Copy {
    const ZERO: Self;
    const ONE: Self;
}

#[snippet("segtree")]
macro_rules! impl_identity {
    ($($target_type:ty),+ $(,)?) => {
        $(
            impl Identity for $target_type {
                const ZERO: $target_type = 0;
                const ONE: $target_type = 1;
            }
        )+
    }
}

#[snippet("segtree")]
impl_identity!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128);

#[snippet("segtree")]
trait Monoid {
    type S: Clone;
    const ID: Self::S;

    fn product(lhs: Self::S, rhs: Self::S) -> Self::S;
}

#[snippet("segtree")]
struct AddMonoid<T>(PhantomData<T>);

#[snippet("segtree")]
impl<T: Identity + std::ops::Add<Output = T>> Monoid for AddMonoid<T> {
    type S = T;
    const ID: Self::S = T::ZERO;

    fn product(lhs: T, rhs: T) -> T {
        lhs + rhs
    }
}

#[snippet("segtree")]
struct SegTree<T: Monoid> {
    orig_len: usize,
    padded_len: usize,
    tree: Vec<T::S>,
}

fn ceil_log2(value: usize) -> usize {
    (usize::BITS - (value - 1).leading_zeros()) as usize
}

#[snippet("segtree")]
impl<M: Monoid> From<Vec<M::S>> for SegTree<M> {
    fn from(vec: Vec<M::S>) -> Self {
        let orig_len = vec.len();
        let padded_len = 1 << ceil_log2(vec.len());
        let mut tree = vec![M::ID; padded_len * 2];

        for (i, x) in vec.into_iter().enumerate() {
            tree[padded_len + i] = x;
        }

        let mut segtree = SegTree {
            orig_len,
            padded_len,
            tree,
        };

        for i in (1..padded_len).rev() {
            segtree.update_single(i);
        }

        segtree
    }
}

#[snippet("segtree")]
impl<M: Monoid> SegTree<M> {
    fn new(len: usize) -> Self {
        vec![M::ID; len].into()
    }

    fn update_single(&mut self, index: usize) {
        if self.padded_len <= index {
            return;
        }

        self.tree[index] = M::product(
            self.tree[index * 2].clone(),
            self.tree[index * 2 + 1].clone(),
        );
    }

    fn update(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        self.update_single(index);
        self.update(index / 2);
    }

    fn set(&mut self, index: usize, value: M::S) {
        assert!(index < self.orig_len);
        self.tree[self.padded_len + index] = value;
        self.update(self.padded_len + index);
    }

    fn get(&self, index: usize) -> &M::S {
        assert!(index < self.orig_len);
        &self.tree[self.padded_len + index]
    }

    fn product_with_segment_range(
        &self,
        l: usize,
        r: usize,
        seg_l: usize,
        seg_r: usize,
        index: usize,
    ) -> M::S {
        if l <= seg_l && seg_r <= r {
            self.tree[index].clone()
        } else if seg_r <= l || r <= seg_l {
            M::ID
        } else {
            let seg_mid = (seg_l + seg_r) / 2;
            let left = self.product_with_segment_range(l, r, seg_l, seg_mid, index * 2);
            let right = self.product_with_segment_range(l, r, seg_mid, seg_r, index * 2 + 1);
            M::product(left, right)
        }
    }

    fn product(&self, l: usize, r: usize) -> M::S {
        self.product_with_segment_range(l, r, 0, self.padded_len, 1)
    }

    fn product_all(&self) -> M::S {
        self.tree[1].clone()
    }
}

#[test]
fn test_segtree() {
    let mut segtree = SegTree::<AddMonoid<usize>>::from(vec![10, 20, 30, 40, 50]);
    assert_eq!(segtree.product_all(), 150);
    assert_eq!(segtree.product(0, 3), 60);
    assert_eq!(segtree.product(2, 5), 120);
    assert_eq!(*segtree.get(2), 30);
    segtree.set(2, 100);
    assert_eq!(segtree.product_all(), 220);
    assert_eq!(segtree.product(0, 3), 130);
    assert_eq!(segtree.product(2, 5), 190);
    assert_eq!(*segtree.get(2), 100);
    let mut segtree = SegTree::<AddMonoid<usize>>::new(5);
    segtree.set(0, 10);
    segtree.set(4, 50);
    segtree.set(1, 20);
    segtree.set(3, 40);
    segtree.set(2, 30);
    assert_eq!(segtree.product_all(), 150);
    assert_eq!(segtree.product(0, 3), 60);
    assert_eq!(segtree.product(2, 5), 120);
}
