use cargo_snippet::snippet;

#[snippet("subseq")]
struct SubSeqIter<'a, T> {
    sequence: &'a [T],
    mask: usize,
}

#[snippet("subseq")]
impl<'a, T: 'a> Iterator for SubSeqIter<'a, T> {
    type Item = SubSeq<'a, T>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.mask >= (1 << self.sequence.len()) {
            return None;
        }
        let subseq = SubSeq {
            sequence: &self.sequence,
            mask: self.mask,
        };
        self.mask += 1;
        Some(subseq)
    }
}

#[snippet("subseq")]
struct SubSeq<'a, T> {
    sequence: &'a [T],
    mask: usize,
}

#[snippet("subseq")]
impl<'a, T> Iterator for SubSeq<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        while self.mask != 0 && self.mask & 1 == 0 && !self.sequence.is_empty() {
            self.mask >>= 1;
            self.sequence = &self.sequence[1..];
        }
        if self.mask & 1 == 1 && !self.sequence.is_empty() {
            self.mask ^= 1;
            Some(&self.sequence[0])
        } else {
            None
        }
    }
}

#[snippet("subseq")]
trait SubSeqable<'a, T: 'a> {
    fn all_subseqs(&self) -> SubSeqIter<T>;
}

#[snippet("subseq")]
impl<'a, T: 'a> SubSeqable<'a, T> for [T] {
    fn all_subseqs(&self) -> SubSeqIter<T> {
        SubSeqIter {
            sequence: self,
            mask: 0,
        }
    }
}

#[test]
fn test_subseq() {
    let v = vec![1, 2, 4];
    assert_eq!(
        v.all_subseqs()
            .map(|s| s.sum::<usize>())
            .collect::<Vec<_>>(),
        (0..8).collect::<Vec<_>>()
    );
    assert_eq!(
        v.all_subseqs()
            .map(|x| x.copied().collect::<Vec<_>>())
            .collect::<Vec<_>>(),
        vec![
            vec![],
            vec![1],
            vec![2],
            vec![1, 2],
            vec![4],
            vec![1, 4],
            vec![2, 4],
            vec![1, 2, 4]
        ]
    );
}
