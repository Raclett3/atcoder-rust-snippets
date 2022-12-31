use super::{ceil_log2, Identity, Monoid};
use cargo_snippet::snippet;
use std::marker::PhantomData;

#[snippet("lazy_segtree")]
trait Morphism<S: Clone> {
    type F: Clone;
    const ID: Self::F;

    fn composition(lhs: Self::F, rhs: Self::F) -> Self::F;
    fn apply(f: Self::F, x: S) -> S;
}

#[snippet("lazy_segtree")]
struct AddMorphism<S>(PhantomData<S>);

#[snippet("segtree")]
impl<S: Identity + std::ops::Add<Output = S>> Morphism<S> for AddMorphism<S> {
    type F = S;
    const ID: Self::F = S::ZERO;

    fn composition(lhs: Self::F, rhs: Self::F) -> Self::F {
        lhs + rhs
    }

    fn apply(f: Self::F, x: S) -> S {
        x + f
    }
}

#[snippet("lazy_segtree")]
#[derive(Clone, Copy, PartialEq, Debug)]
struct ValueWithWidth {
    value: isize,
    width: isize,
}

#[snippet("lazy_segtree")]
impl std::ops::Add for ValueWithWidth {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        ValueWithWidth {
            value: self.value + rhs.value,
            width: self.width + rhs.width,
        }
    }
}

#[snippet("lazy_segtree")]
impl std::ops::Add<isize> for ValueWithWidth {
    type Output = Self;

    fn add(self, rhs: isize) -> Self {
        ValueWithWidth {
            value: self.value + rhs * self.width,
            width: self.width,
        }
    }
}

#[snippet("lazy_segtree")]
impl Identity for ValueWithWidth {
    const ZERO: Self = ValueWithWidth { value: 0, width: 0 };
    const ONE: Self = ValueWithWidth { value: 1, width: 1 };
}

#[snippet("lazy_segtree")]
impl ValueWithWidth {
    fn new(value: isize) -> Self {
        ValueWithWidth { value, width: 1 }
    }
}

#[snippet("lazy_segtree")]
struct AddMorphismWithWidth;

#[snippet("segtree")]
impl Morphism<ValueWithWidth> for AddMorphismWithWidth {
    type F = isize;
    const ID: Self::F = 0;

    fn composition(lhs: Self::F, rhs: Self::F) -> Self::F {
        lhs + rhs
    }

    fn apply(f: Self::F, x: ValueWithWidth) -> ValueWithWidth {
        x + f
    }
}

#[snippet("lazy_segtree")]
struct LazySegTree<Mono: Monoid, Morph: Morphism<Mono::S>> {
    orig_len: usize,
    padded_len: usize,
    tree: Vec<Mono::S>,
    lazy_tree: Vec<Morph::F>,
}

#[snippet("lazy_segtree")]
impl<Mono: Monoid, Morph: Morphism<Mono::S>> From<Vec<Mono::S>> for LazySegTree<Mono, Morph> {
    fn from(vec: Vec<Mono::S>) -> Self {
        let orig_len = vec.len();
        let padded_len = 1 << ceil_log2(vec.len());
        let mut tree = vec![Mono::ID; padded_len * 2];

        for (i, x) in vec.into_iter().enumerate() {
            tree[padded_len + i] = x;
        }

        let lazy_tree = vec![Morph::ID; padded_len * 2];

        let mut lazy_segtree = LazySegTree {
            orig_len,
            padded_len,
            tree,
            lazy_tree,
        };

        for i in (1..padded_len).rev() {
            lazy_segtree.update_single(i);
        }

        lazy_segtree
    }
}

#[snippet("lazy_segtree")]
impl<Mono: Monoid, Morph: Morphism<Mono::S>> LazySegTree<Mono, Morph> {
    fn new(len: usize) -> Self {
        vec![Mono::ID; len].into()
    }

    fn update_single(&mut self, index: usize) {
        if self.padded_len <= index {
            return;
        }

        self.tree[index] = Mono::product(
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

    fn eval(&mut self, index: usize) {
        self.tree[index] = Morph::apply(self.lazy_tree[index].clone(), self.tree[index].clone());

        if index * 2 < self.padded_len * 2 {
            self.lazy_tree[index * 2] = Morph::composition(
                self.lazy_tree[index].clone(),
                self.lazy_tree[index * 2].clone(),
            );
            self.lazy_tree[index * 2 + 1] = Morph::composition(
                self.lazy_tree[index].clone(),
                self.lazy_tree[index * 2 + 1].clone(),
            );
        }

        self.lazy_tree[index] = Morph::ID;
    }

    fn recursive_eval(&mut self, index: usize) {
        if index == 0 {
            return;
        }

        self.recursive_eval(index / 2);
        self.eval(index);
    }

    fn set(&mut self, index: usize, value: Mono::S) {
        assert!(index < self.orig_len);
        self.recursive_eval(self.padded_len + index);
        self.tree[self.padded_len + index] = value;
        self.update(self.padded_len + index);
    }

    fn get(&mut self, index: usize) -> &Mono::S {
        assert!(index < self.orig_len);
        self.recursive_eval(self.padded_len + index);
        &self.tree[self.padded_len + index]
    }

    fn product_with_segment_range(
        &mut self,
        l: usize,
        r: usize,
        seg_l: usize,
        seg_r: usize,
        index: usize,
    ) -> Mono::S {
        if l <= seg_l && seg_r <= r {
            self.eval(index);
            self.tree[index].clone()
        } else if seg_r <= l || r <= seg_l {
            Mono::ID
        } else {
            self.eval(index);
            let seg_mid = (seg_l + seg_r) / 2;
            let left = self.product_with_segment_range(l, r, seg_l, seg_mid, index * 2);
            let right = self.product_with_segment_range(l, r, seg_mid, seg_r, index * 2 + 1);
            Mono::product(left, right)
        }
    }

    fn product(&mut self, l: usize, r: usize) -> Mono::S {
        self.product_with_segment_range(l, r, 0, self.padded_len, 1)
    }

    fn apply_with_segment_range(
        &mut self,
        l: usize,
        r: usize,
        f: &Morph::F,
        seg_l: usize,
        seg_r: usize,
        index: usize,
    ) {
        self.eval(index);

        if l <= seg_l && seg_r <= r {
            self.lazy_tree[index] = Morph::composition(f.clone(), self.lazy_tree[index].clone());
            self.eval(index);
        } else if l < seg_r && seg_l < r {
            let seg_mid = (seg_l + seg_r) / 2;
            self.apply_with_segment_range(l, r, f, seg_l, seg_mid, index * 2);
            self.apply_with_segment_range(l, r, f, seg_mid, seg_r, index * 2 + 1);
            self.update_single(index);
        }
    }

    fn apply(&mut self, l: usize, r: usize, f: Morph::F) {
        self.apply_with_segment_range(l, r, &f, 0, self.padded_len, 1);
    }

    fn product_all(&mut self) -> Mono::S {
        self.eval(1);
        self.tree[1].clone()
    }
}

#[test]
fn test_lazy_segtree() {
    use super::AddMonoid;

    fn print_segtree(segtree: &LazySegTree<AddMonoid<ValueWithWidth>, AddMorphismWithWidth>) {
        for p in 0..4 {
            eprintln!(
                "{:?}",
                &segtree.tree[(1 << p)..(1 << (p + 1))]
                    .iter()
                    .map(|ValueWithWidth { value, width }| (value, width))
                    .collect::<Vec<_>>()
            );
            eprintln!("{:?}", &segtree.lazy_tree[(1 << p)..(1 << (p + 1))]);
        }
        eprintln!();
    }

    let mut segtree = LazySegTree::<AddMonoid<ValueWithWidth>, AddMorphismWithWidth>::from(
        vec![10, 20, 30, 40, 50]
            .into_iter()
            .map(ValueWithWidth::new)
            .collect::<Vec<_>>(),
    );

    assert_eq!(segtree.product_all().value, 150);
    assert_eq!(segtree.product(0, 3).value, 60);
    assert_eq!(segtree.product(2, 5).value, 120);
    assert_eq!(segtree.get(2).value, 30);
    segtree.set(2, ValueWithWidth::new(100)); // [10, 20, 100, 40, 50]
    assert_eq!(segtree.product_all().value, 220);
    assert_eq!(segtree.product(0, 3).value, 130);
    assert_eq!(segtree.product(2, 5).value, 190);
    assert_eq!(segtree.get(2).value, 100);
    segtree.apply(1, 4, 20); // [10, 40, 120, 60, 50]
    assert_eq!(segtree.product_all().value, 280);
    assert_eq!(segtree.product(0, 3).value, 170);
    assert_eq!(segtree.product(2, 5).value, 230);
    assert_eq!(segtree.get(2).value, 120);
    segtree.apply(0, 4, 30); // [40, 70, 150, 90, 50]
    segtree.apply(1, 5, 50); // [40, 120, 200, 140, 100]
    assert_eq!(segtree.product_all().value, 600);
    assert_eq!(segtree.product(0, 3).value, 360);
    assert_eq!(segtree.product(2, 5).value, 440);
    assert_eq!(segtree.get(2).value, 200);
}
