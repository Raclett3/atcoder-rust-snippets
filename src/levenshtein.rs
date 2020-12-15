#![allow(clippy::needless_range_loop)]

use cargo_snippet::snippet;

#[snippet("levenshtein")]
fn levenshtein<T: Eq>(a: &[T], b: &[T]) -> usize {
    let mut dp = vec![vec![0; b.len() + 1]; a.len() + 1];
    for i in 0..=a.len() {
        for j in 0..=b.len() {
            if i == 0 {
                dp[i][j] = j;
                continue;
            }

            if j == 0 {
                dp[i][j] = i;
                continue;
            }

            dp[i][j] = (dp[i - 1][j] + 1).min(dp[i][j - 1] + 1);

            dp[i][j] = dp[i][j].min(dp[i - 1][j - 1] + (a[i - 1] != b[j - 1]) as usize);
        }
    }

    dp[a.len()][b.len()]
}

#[test]
fn test_levenshtein() {
    assert_eq!(levenshtein(b"abc", b"abc"), 0);
    assert_eq!(levenshtein(b"abc", b"abcd"), 1);
    assert_eq!(levenshtein(b"abc", b"abdc"), 1);
    assert_eq!(levenshtein(b"abc", b"abd"), 1);
    assert_eq!(levenshtein(b"abc", b"dbc"), 1);
    assert_eq!(levenshtein(b"", b"abcd"), 4);
}
