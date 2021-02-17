use cargo_snippet::snippet;

#[snippet("range_union")]
#[derive(Clone, Debug)]
struct RangeUnion {
    map: std::collections::BTreeMap<isize, isize>,
    cum_map: std::collections::BTreeMap<isize, isize>,
    cum_applied: bool,
}

#[snippet("range_union")]
impl RangeUnion {
    #[allow(clippy::new_without_default)]
    fn new() -> Self {
        RangeUnion {
            map: std::collections::BTreeMap::new(),
            cum_map: std::collections::BTreeMap::new(),
            cum_applied: true,
        }
    }

    fn inner_union_range(&mut self, left: isize, right: isize) {
        self.cum_applied = false;
        self.map.entry(left).and_modify(|x| *x += 1).or_insert(1);
        self.map.entry(right).and_modify(|x| *x -= 1).or_insert(-1);
    }

    fn add_range_inclusive(&mut self, left: isize, right: isize) {
        if left <= right {
            self.inner_union_range(left, right + 1);
        } else {
            self.inner_union_range(right, left + 1);
        }
    }

    fn add_range_exclusive(&mut self, left: isize, right: isize) {
        if left <= right {
            self.inner_union_range(left, right);
        } else {
            self.inner_union_range(right + 1, left + 1);
        }
    }

    fn apply_cum(&mut self) {
        if self.cum_applied {
            return;
        }

        let mut acc = 0;
        let mut cum_map = std::collections::BTreeMap::new();

        for (&i, &x) in self.map.iter() {
            acc += x;
            cum_map.insert(i, acc);
        }

        self.cum_map = cum_map;
        self.cum_applied = true;
    }

    fn includes(&mut self, x: isize) -> bool {
        self.apply_cum();
        eprintln!("{:?}", self.cum_map);
        self.cum_map
            .range(..=x)
            .map(|(_, &x)| x)
            .next_back()
            .unwrap_or(0)
            > 0
    }

    fn len(&mut self) -> isize {
        self.apply_cum();
        let mut length = 0;

        for ((&left, &x), (&right, _)) in self.cum_map.iter().zip(self.cum_map.iter().skip(1)) {
            if x > 0 {
                length += right - left;
            }
        }

        length
    }
}

#[test]
fn test_range_union() {
    let mut r_union = RangeUnion::new();
    r_union.add_range_exclusive(-5, -1);
    r_union.add_range_inclusive(0, 3);
    r_union.add_range_inclusive(2, 6);
    r_union.add_range_inclusive(19, 10);
    assert_eq!(r_union.len(), 21);
    assert!(r_union.includes(4));
    assert!(r_union.includes(13));
    assert!(!r_union.includes(8));
    assert!(!r_union.includes(-1));
}
