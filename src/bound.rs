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

#[snippet("bound")]
pub fn usize_bound<F: Fn(usize) -> bool>(min: usize, max: usize, f: F) -> usize {
    let exclusive_max = if min <= max {
        max as isize + 1
    } else {
        max as isize - 1
    };
    bound(min as isize, exclusive_max, |x| f(x as usize)) as usize
}

#[snippet("bound")]
pub fn float_bound<F: Fn(f64) -> bool>(min: f64, max: f64, f: F) -> f64 {
    let mut ok = min;
    let mut ng = max;
    for _ in 0..100 {
        let mid = (ok + ng) / 2.0;
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
    use approx::*;
    
    assert_eq!(bound(0, 101, |x| x <= 10), 10);
    assert_eq!(bound(0, 101, |x| x <= 100), 100);
    assert_eq!(bound(0, 101, |x| x <= 200), 100);
    assert_eq!(bound(100, -1, |x| x >= 20), 20);
    assert_eq!(bound(100, -1, |x| x >= 0), 0);
    assert_eq!(bound(100, -1, |x| x >= -1), 0);
    assert_eq!(usize_bound(0, 100, |x| x <= 10), 10);
    assert_eq!(usize_bound(100, 0, |x| x >= 30), 30);
    assert_eq!(usize_bound(100, 10, |x| x >= 5), 10);
    assert_relative_eq!(float_bound(-10.0, 2.0, |x| x <= 1.0), 1.0);
    assert_relative_eq!(float_bound(-10.0, 2.0, |x| x <= 100.0), 2.0);
    assert_relative_eq!(float_bound(-10.0, 2.0, |x| x <= -100.0), -10.0);
}
