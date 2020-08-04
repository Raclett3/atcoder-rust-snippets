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
