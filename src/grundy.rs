use cargo_snippet::snippet;

#[snippet("grundy")]
use std::collections::{BTreeMap, BTreeSet};

#[snippet("grundy")]
fn mex(set: &BTreeSet<usize>) -> usize {
    for i in 0.. {
        if !set.contains(&i) {
            return i;
        }
    }
    unreachable!();
}

#[snippet("grundy")]
fn all_grundy<T: Game>(init: T) -> BTreeMap<T, usize> {
    let mut grundy_map = BTreeMap::new();
    next_grundy(&mut grundy_map, init);
    grundy_map
}

#[snippet("grundy")]
fn next_grundy<T: Game>(grundy_map: &mut BTreeMap<T, usize>, current: T) -> usize {
    if let Some(&grundy) = grundy_map.get(&current) {
        return grundy;
    }

    if current.is_terminal() {
        grundy_map.insert(current, 0);
        return 0;
    }

    let mut grundy_set = BTreeSet::new();
    for state in current.next_states() {
        grundy_set.insert(next_grundy(grundy_map, state));
    }

    let grundy = mex(&grundy_set);
    grundy_map.insert(current, grundy);
    grundy
}

#[snippet("grundy")]
trait Game: Ord + Sized {
    fn next_states(&self) -> Vec<Self>;
    fn is_terminal(&self) -> bool;
}

#[snippet("grundy")]
fn first_wins<T: Game + Clone>(init: T) -> bool {
    let grundy = all_grundy(init.clone());
    *grundy.get(&init).unwrap() != 0
}

#[test]
fn test_grundy() {
    impl Game for Vec<usize> {
        fn next_states(&self) -> Vec<Self> {
            self.iter()
                .enumerate()
                .flat_map(|(i, &heap)| {
                    (0..heap).map(move |x| {
                        let mut cloned = self.clone();
                        cloned[i] = x;
                        cloned
                    })
                })
                .collect()
        }

        fn is_terminal(&self) -> bool {
            self.iter().all(|&x| x == 0)
        }
    }

    let grundy = all_grundy(vec![4, 5, 6]);
    eprintln!("{:?}", grundy);
    for i in 0..=4 {
        for j in 0..=5 {
            for k in 0..=6 {
                assert_eq!(*grundy.get(&vec![i, j, k]).unwrap(), i ^ j ^ k);
            }
        }
    }
    
    assert!(first_wins(vec![10, 2, 5]));
    assert!(first_wins(vec![6, 6, 6]));
    assert!(first_wins(vec![5, 6, 7, 8]));
    assert!(!first_wins(vec![7, 4, 3]));
    assert!(!first_wins(vec![7, 7, 7, 7]));
}
