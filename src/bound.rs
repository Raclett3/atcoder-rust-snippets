use cargo_snippet::snippet;

#[snippet("bound")]
pub fn bound<F: Fn(isize) -> bool>(min: isize, max: isize, f: F) -> isize {
    let mut ok = min;
    let mut ng = max;
    while (ng - ok).abs() > 1 {
        let mid = (ok + ng) / 2;
        if f(mid) {
            ok = mid;
        } else {
            ng = mid;
        }
    }
    ok
}

#[test]
fn test_bound() {
    assert_eq!(bound(0, 101, |x| x <= 10), 10);
    assert_eq!(bound(0, 101, |x| x <= 100), 100);
    assert_eq!(bound(0, 101, |x| x <= 200), 100);
    assert_eq!(bound(100, -1, |x| x >= 20), 20);
    assert_eq!(bound(100, -1, |x| x >= 0), 0);
    assert_eq!(bound(100, -1, |x| x >= -1), 0);
}
