use cargo_snippet::snippet;

#[snippet("rng")]
struct RNG(u64);

#[snippet("rng")]
impl RNG {
    fn rand(&mut self) -> u64 {
        let RNG(x) = self;
        *x ^= *x << 13;
        *x ^= *x >> 7;
        *x ^= *x << 17;
        *x
    }
}

#[test]
fn test_rng() {
    use std::collections::BTreeSet;
    let mut set = BTreeSet::new();
    let mut rng = RNG(12345);

    for _ in 0..100000 {
        let num = rng.rand();
        assert!(!set.contains(&num));
        set.insert(num);
    }
}
