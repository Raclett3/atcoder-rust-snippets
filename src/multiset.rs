use cargo_snippet::snippet;

#[snippet("multiset")]
use std::iter::FromIterator;

#[snippet("multiset")]
use std::collections::{btree_map::Range, BTreeMap};

#[snippet("multiset")]
use std::ops::RangeBounds;

#[snippet("multiset")]
struct BTreeMultiSet<T: Ord> {
    map: BTreeMap<T, usize>,
}

#[snippet("multiset")]
impl<T: Ord> BTreeMultiSet<T> {
    #[allow(clippy::new_without_default)]
    fn new() -> Self {
        Self {
            map: BTreeMap::new(),
        }
    }

    fn insert(&mut self, value: T) -> &mut usize {
        self.map.entry(value).and_modify(|x| *x += 1).or_insert(1)
    }

    fn remove_one(&mut self, value: &T) -> bool {
        let (key_exists, remove_key) = if let Some(count) = self.map.get_mut(value) {
            *count -= 1;
            (true, *count == 0)
        } else {
            (false, false)
        };
        if remove_key {
            self.map.remove(value);
        }
        key_exists
    }

    fn remove_all(&mut self, value: &T) -> usize {
        self.map.remove(value).unwrap_or(0)
    }

    fn count(&self, value: &T) -> usize {
        self.map.get(value).copied().unwrap_or(0)
    }

    fn is_disjoint(&self, other: &BTreeMultiSet<T>) -> bool {
        self.map.iter().all(|(key, _)| other.count(key) == 0)
    }

    fn is_subset(&self, other: &BTreeMultiSet<T>) -> bool {
        self.map
            .iter()
            .all(|(key, &count)| count <= other.count(key))
    }

    fn is_empty(&self) -> bool {
        self.map.is_empty()
    }

    fn is_superset(&self, other: &BTreeMultiSet<T>) -> bool {
        other.is_subset(self)
    }

    fn range<R: RangeBounds<T>>(&self, range: R) -> BTreeMultiSetIter<'_, T> {
        BTreeMultiSetIter {
            iter: self.map.range(range),
            current_item: None,
            count: 0,
        }
    }

    fn iter(&self) -> BTreeMultiSetIter<'_, T> {
        self.range(..)
    }
}

#[snippet("multiset")]
impl<T: Ord> FromIterator<T> for BTreeMultiSet<T> {
    fn from_iter<I: IntoIterator<Item = T>>(iter: I) -> Self {
        let mut set = BTreeMultiSet::new();

        for value in iter {
            set.insert(value);
        }

        set
    }
}

#[snippet("multiset")]
struct BTreeMultiSetIter<'a, T: Ord> {
    iter: Range<'a, T, usize>,
    current_item: Option<&'a T>,
    count: usize,
}

#[snippet("multiset")]
impl<'a, T: Ord> Iterator for BTreeMultiSetIter<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let item = if self.count == 0 {
            let (next, count) = self
                .iter
                .next()
                .map(|(item, &count)| (Some(item), count))
                .unwrap_or((None, 0));
            self.current_item = next;
            self.count = count;
            next
        } else {
            self.current_item
        };
        if item.is_some() {
            self.count -= 1;
        }
        item
    }
}

#[snippet("multiset")]
impl<'a, T: Ord> std::iter::DoubleEndedIterator for BTreeMultiSetIter<'a, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        let item = if self.count == 0 {
            let (next, count) = self
                .iter
                .next_back()
                .map(|(item, &count)| (Some(item), count))
                .unwrap_or((None, 0));
            self.current_item = next;
            self.count = count;
            next
        } else {
            self.current_item
        };
        if item.is_some() {
            self.count -= 1;
        }
        item
    }
}

#[test]
fn test_multiset() {
    let mut set = BTreeMultiSet::from_iter([0, 1, 2, 3, 1, 2, 3, 2, 3, 3].iter().copied());
    assert_eq!(set.count(&0), 1);
    assert_eq!(set.count(&2), 3);
    assert_eq!(set.count(&4), 0);
    assert!(set.is_disjoint(&BTreeMultiSet::from_iter([4, 5, 6].iter().copied())));
    assert!(!set.is_disjoint(&BTreeMultiSet::from_iter([0, 1, 2].iter().copied())));
    assert!(set.is_superset(&BTreeMultiSet::from_iter([0, 1, 2].iter().copied())));
    assert!(set.is_superset(&BTreeMultiSet::from_iter([0, 1, 2, 3].iter().copied())));
    assert!(set.is_superset(&BTreeMultiSet::from_iter([3, 3, 3, 3].iter().copied())));
    assert!(!set.is_superset(&BTreeMultiSet::from_iter([0, 0].iter().copied())));
    assert!(!set.is_superset(&BTreeMultiSet::from_iter([0, 1, 2, 4].iter().copied())));
    assert!(!set.is_superset(&BTreeMultiSet::from_iter([4].iter().copied())));
    assert_eq!(
        set.iter().copied().collect::<Vec<_>>(),
        vec![0, 1, 1, 2, 2, 2, 3, 3, 3, 3]
    );
    assert_eq!(
        set.range(2..).copied().collect::<Vec<_>>(),
        vec![2, 2, 2, 3, 3, 3, 3]
    );
    assert_eq!(
        set.range(2..).rev().copied().collect::<Vec<_>>(),
        vec![3, 3, 3, 3, 2, 2, 2]
    );
    set.insert(0);
    assert_eq!(set.count(&0), 2);
    set.remove_one(&2);
    assert_eq!(set.count(&2), 2);
    set.remove_all(&3);
    assert_eq!(set.count(&3), 0);
    assert!(!set.is_empty());
    assert!(BTreeMultiSet::<()>::new().is_empty());
    let mut to_be_empty = BTreeMultiSet::new();
    to_be_empty.insert(1);
    to_be_empty.remove_all(&1);
    assert!(to_be_empty.is_empty());
}
