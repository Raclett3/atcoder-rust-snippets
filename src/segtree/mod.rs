mod lazy_segtree;
mod segtree;

use cargo_snippet::snippet;

#[snippet("segtree")]
#[snippet("lazy_segtree")]
use std::marker::PhantomData;

#[snippet("segtree")]
#[snippet("lazy_segtree")]
trait Identity: Copy {
    const ZERO: Self;
    const ONE: Self;
}

#[snippet("segtree")]
#[snippet("lazy_segtree")]
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
#[snippet("lazy_segtree")]
impl_identity!(usize, u8, u16, u32, u64, u128, isize, i8, i16, i32, i64, i128);

#[snippet("segtree")]
#[snippet("lazy_segtree")]
trait Monoid {
    type S: Clone;
    const ID: Self::S;

    fn product(lhs: Self::S, rhs: Self::S) -> Self::S;
}

#[snippet("segtree")]
#[snippet("lazy_segtree")]
struct AddMonoid<T>(PhantomData<T>);

#[snippet("segtree")]
#[snippet("lazy_segtree")]
impl<T: Identity + std::ops::Add<Output = T>> Monoid for AddMonoid<T> {
    type S = T;
    const ID: Self::S = T::ZERO;

    fn product(lhs: T, rhs: T) -> T {
        lhs + rhs
    }
}

#[snippet("segtree")]
#[snippet("lazy_segtree")]
fn ceil_log2(value: usize) -> usize {
    (usize::BITS - (value - 1).leading_zeros()) as usize
}
