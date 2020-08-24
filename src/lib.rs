#![allow(dead_code)]
#![allow(unused_macros)]

mod bits;
mod bound;
mod geometric;
mod graph;
mod modint;
mod produce;
mod safe_index;

#[test]
fn test_modint() {
    use modint::*;
    assert_eq!(ModInt(MOD - 2), ModInt(MOD - 1) + ModInt(MOD - 1));
    assert_eq!(ModInt(MOD - 1), ModInt(MOD - 2) - ModInt(MOD - 1));
    assert_eq!(ModInt(9), ModInt(MOD - 3) * ModInt(MOD - 3));
    assert_eq!(ModInt(898961331), ModInt(2).pow(50));
    assert_eq!(ModInt(12345), ModInt(12345) / ModInt(67890) * ModInt(67890));
    let mut fact = ModIntFact::new();
    assert_eq!(ModInt(227020758), fact.fact(13));
    assert_eq!(ModInt(1), fact.fact(127) * fact.fact_inv(127));
    assert_eq!(ModInt(184756), fact.ncr(20, 10));
    assert_eq!(ModInt(360360), fact.npr(15, 5));
    assert_eq!(ModInt(2002), fact.nhr(10, 5));
    assert_eq!(ModInt(12345), mint(12345));
    assert_eq!(ModInt(123), mint(MOD * 2 + 123));
    assert_eq!("12345", format!("{}", ModInt(12345)));
}

#[test]
fn test_produce() {
    use produce::produce;

    let producer = produce(1, |acc| Some(acc * 2 % 11), true);
    let actual: Vec<usize> = producer.take(10).collect();
    assert_eq!(vec![1, 2, 4, 8, 5, 10, 9, 7, 3, 6], actual);

    let producer = produce(
        1,
        |acc| {
            let next = acc * 3;
            if next <= 100 {
                Some(next)
            } else {
                None
            }
        },
        true,
    );
    let actual: Vec<usize> = producer.collect();
    assert_eq!(vec![1, 3, 9, 27, 81], actual);

    let producer = produce(
        1,
        |acc| {
            let next = acc * 2;
            if next <= 100 {
                Some(next)
            } else {
                None
            }
        },
        false,
    );
    let actual: Vec<usize> = producer.collect();
    assert_eq!(vec![2, 4, 8, 16, 32, 64], actual);
}

#[test]
fn test_bound() {
    use bound::bound;
    assert_eq!(bound(0, 101, |x| x <= 10), 10);
    assert_eq!(bound(0, 101, |x| x <= 100), 100);
    assert_eq!(bound(0, 101, |x| x <= 200), 100);
    assert_eq!(bound(100, -1, |x| x >= 20), 20);
    assert_eq!(bound(100, -1, |x| x >= 0), 0);
    assert_eq!(bound(100, -1, |x| x >= -1), 0);
}

#[test]
fn graph() {
    use graph::*;
    let mut graph = Graph::new(8);
    graph.edge_undirected_costed(0, 7, 2);
    graph.edge_undirected_costed(1, 7, 4);
    graph.edge_undirected_costed(0, 3, 9);
    graph.edge_undirected_costed(1, 3, 3);
    graph.edge_undirected_costed(0, 4, 13);
    graph.edge_undirected_costed(3, 4, 3);
    graph.edge_undirected_costed(3, 5, 5);
    graph.edge_undirected_costed(1, 5, 6);
    graph.edge_undirected_costed(2, 4, 8);
    graph.edge_undirected_costed(2, 3, 11);
    graph.edge_undirected_costed(2, 6, 4);
    graph.edge_undirected_costed(3, 6, 6);
    graph.edge_undirected_costed(5, 6, 5);
    assert_eq!(vec![0, 6, 19, 9, 12, 12, 15, 2], graph.dijkstra(0));
    let mut graph = Graph::new(6);
    graph.edge_undirected_costed(0, 1, 5);
    graph.edge_undirected_costed(0, 4, 15);
    graph.edge_undirected_costed(1, 2, 1);
    graph.edge_undirected_costed(2, 3, 2);
    graph.edge_undirected_costed(2, 4, 9);
    graph.edge_undirected_costed(3, 4, 6);
    let inf = std::isize::MAX;
    let expected = vec![
        vec![0, 5, 6, 8, 14, inf],
        vec![5, 0, 1, 3, 9, inf],
        vec![6, 1, 0, 2, 8, inf],
        vec![8, 3, 2, 0, 6, inf],
        vec![14, 9, 8, 6, 0, inf],
        vec![inf, inf, inf, inf, inf, 0],
    ];
    assert_eq!(expected, graph.warshall_floyd());
}

#[test]
fn test_safe_index() {
    use safe_index::*;

    let mut v = vec![10, 20, 30, 40, 50];
    assert_eq!(v.at(0), Some(&10));
    assert_eq!(v.at(-2), None);
    assert_eq!(v.at(5), None);
    assert_eq!(v.at(4), Some(&50));
    assert_eq!(v.at_mut(-1), None);
    assert_eq!(v.at_mut(5), None);
    *v.at_mut(2).unwrap() = 60;
    assert_eq!(v.at(2), Some(&60));
}
