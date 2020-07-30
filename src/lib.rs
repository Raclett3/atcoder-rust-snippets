use cargo_snippet::snippet;

#[snippet("bits")]
struct Bits(usize);

#[snippet("bits")]
impl Bits {
    fn at(&self, index: usize) -> bool {
        self.0 & 1 << index > 0
    }

    fn set_bits(&self) -> usize {
        let mut n = self.0;
        let mut count = 0;
        while n != 0 {
            n &= n - 1;
            count += 1;
        }
        count
    }
}

#[snippet("bits")]
macro_rules! n_bits_range {
    ($bits:expr) => {
        (0..(1 << $bits)).map(Bits)
    };
}

#[snippet("produce")]
struct Produce<T: Copy, F: Fn(T) -> Option<T>> {
    acc: Option<T>,
    func: F,
    include_init: bool,
}

#[snippet("produce")]
fn produce<T: Copy, F: Fn(T) -> Option<T>>(init: T, func: F, include_init: bool) -> Produce<T, F> {
    Produce {
        acc: Some(init),
        func,
        include_init,
    }
}

#[snippet("produce")]
impl<T: Copy, F: Fn(T) -> Option<T>> std::iter::Iterator for Produce<T, F> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(acc) = self.acc {
            if let Some(next) = (self.func)(acc) {
                let current = if self.include_init {
                    acc
                } else {
                    next
                };
                self.acc = Some(next);
                Some(current)
            } else {
                self.acc = None;
                if self.include_init {
                    Some(acc)
                } else {
                    None
                }
            }
        } else {
            None
        }
    }
}

#[snippet("geometric")]
struct Geometric {
    current: isize,
    ratio: isize,
}

#[snippet("geometric")]
impl std::iter::Iterator for Geometric {
    type Item = isize;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.current;
        let (muled, overflowed) = self.current.overflowing_mul(self.ratio);
        if overflowed {
            return None;
        }
        self.current = muled;
        Some(next)
    }
}

#[snippet("geometric")]
impl Geometric {
    fn new(init: isize, ratio: isize) -> Self {
        Self {
            current: init,
            ratio,
        }
    }
}

#[snippet("modint")]
const MOD: usize = 1000000007;

#[snippet("modint")]
#[derive(Copy, Clone, Eq, PartialEq, std::fmt::Debug)]
struct ModInt(usize);

#[snippet("modint")]
impl std::ops::Add for ModInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        ModInt((self.0 + rhs.0) % MOD)
    }
}

#[snippet("modint")]
#[allow(clippy::suspicious_op_assign_impl)]
impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.0 %= MOD;
    }
}

#[snippet("modint")]
impl std::ops::Sub for ModInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        ModInt((self.0 + MOD - rhs.0) % MOD)
    }
}

#[snippet("modint")]
#[allow(clippy::suspicious_op_assign_impl)]
impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 += MOD - rhs.0;
        self.0 %= MOD;
    }
}

#[snippet("modint")]
impl std::ops::Mul for ModInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        ModInt((self.0 * rhs.0) % MOD)
    }
}

#[snippet("modint")]
#[allow(clippy::suspicious_op_assign_impl)]
impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.0 %= MOD;
    }
}

#[snippet("modint")]
impl std::ops::Div for ModInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        ModInt((self.0 * rhs.pow(MOD - 2).0) % MOD)
    }
}

#[snippet("modint")]
#[allow(clippy::suspicious_op_assign_impl)]
impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        self.0 *= rhs.pow(MOD - 2).0;
        self.0 %= MOD;
    }
}

#[snippet("modint")]
impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[snippet("modint")]
impl ModInt {
    fn pow(&self, power: usize) -> ModInt {
        let mut acc_base = *self;
        let mut acc_pow = power;
        let mut res = ModInt(1);
        while acc_pow > 0 {
            if acc_pow & 1 == 1 {
                res *= acc_base;
            }
            acc_base *= acc_base;
            acc_pow >>= 1;
        }
        res
    }
}

#[snippet("modint")]
struct ModIntFact {
    memo: Vec<ModInt>,
    memo_inv: Vec<ModInt>,
}

#[snippet("modint")]
impl ModIntFact {
    fn new(size: usize) -> Self {
        let mut memo = vec![ModInt(0); size + 1];
        memo[0] = ModInt(1);
        for n in 1..=size {
            memo[n] = memo[n - 1] * ModInt(n);
        }
        let memo_inv = memo.iter().map(|x| x.pow(MOD - 2)).collect();
        Self { memo, memo_inv }
    }

    fn fact(&self, n: usize) -> ModInt {
        self.memo[n]
    }

    fn fact_inv(&self, n: usize) -> ModInt {
        self.memo_inv[n]
    }

    fn ncr(&self, n: usize, r: usize) -> ModInt {
        self.memo[n] * self.memo_inv[r] * self.memo_inv[n - r]
    }

    fn npr(&self, n: usize, r: usize) -> ModInt {
        self.memo[n] * self.memo_inv[n - r]
    }

    fn nhr(&self, n: usize, r: usize) -> ModInt {
        self.memo[n + r - 1] * self.memo_inv[r] * self.memo_inv[n - 1]
    }
}

#[snippet("modint")]
impl std::iter::Sum for ModInt {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt(0), |x, y| x + y)
    }
}

#[snippet("graph")]
use std::cmp::*;
#[snippet("graph")]
use std::collections::binary_heap::BinaryHeap;

#[snippet("graph")]
#[derive(Copy, Clone, Eq, PartialEq)]
struct GraphEdge {
    node_to: usize,
    cost: isize,
}

#[snippet("graph")]
impl Ord for GraphEdge {
    fn cmp(&self, other: &GraphEdge) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

#[snippet("graph")]
impl PartialOrd for GraphEdge {
    fn partial_cmp(&self, other: &GraphEdge) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[snippet("graph")]
impl GraphEdge {
    fn new(node_to: usize, cost: isize) -> Self {
        Self { node_to, cost }
    }
}

#[snippet("graph")]
struct Graph {
    pub edges: Vec<Vec<GraphEdge>>,
    pub nodes: usize,
}

#[snippet("graph")]
impl Graph {
    fn new(nodes: usize) -> Self {
        Self {
            edges: vec![vec![]; nodes],
            nodes,
        }
    }

    fn edge_undirected(&mut self, node_a: usize, node_b: usize) {
        self.edges[node_a].push(GraphEdge::new(node_b, 1));
        self.edges[node_b].push(GraphEdge::new(node_a, 1));
    }

    fn edge_directed(&mut self, node_from: usize, node_to: usize) {
        self.edges[node_from].push(GraphEdge::new(node_to, 1));
    }

    fn edge_undirected_costed(&mut self, node_a: usize, node_b: usize, cost: isize) {
        self.edges[node_a].push(GraphEdge::new(node_b, cost));
        self.edges[node_b].push(GraphEdge::new(node_a, cost));
    }

    fn edge_directed_costed(&mut self, node_from: usize, node_to: usize, cost: isize) {
        self.edges[node_from].push(GraphEdge::new(node_to, cost));
    }

    fn dijkstra(&self, node_from: usize) -> Vec<isize> {
        let mut costs = vec![std::isize::MAX; self.nodes];
        let mut heap: BinaryHeap<GraphEdge> = BinaryHeap::new();
        heap.push(GraphEdge::new(node_from, 0));
        while let Some(GraphEdge { cost, node_to }) = heap.pop() {
            if cost >= costs[node_to] {
                continue;
            }
            costs[node_to] = cost;

            for edge in &self.edges[node_to] {
                let next = GraphEdge::new(edge.node_to, cost + edge.cost);
                heap.push(next);
            }
        }
        costs
    }
}

#[test]
fn test_modint() {
    assert_eq!(ModInt(MOD - 2), ModInt(MOD - 1) + ModInt(MOD - 1));
    assert_eq!(ModInt(MOD - 1), ModInt(MOD - 2) - ModInt(MOD - 1));
    assert_eq!(ModInt(9), ModInt(MOD - 3) * ModInt(MOD - 3));
    assert_eq!(ModInt(898961331), ModInt(2).pow(50));
    assert_eq!(ModInt(12345), ModInt(12345) / ModInt(67890) * ModInt(67890));
    let fact = ModIntFact::new(1024);
    assert_eq!(ModInt(227020758), fact.fact(13));
    assert_eq!(ModInt(1), fact.fact(127) * fact.fact_inv(127));
    assert_eq!(ModInt(184756), fact.ncr(20, 10));
    assert_eq!(ModInt(360360), fact.npr(15, 5));
    assert_eq!(ModInt(2002), fact.nhr(10, 5));
    assert_eq!("12345", format!("{}", ModInt(12345)));
}

#[test]
fn test_produce() {
    let producer = produce(1, |acc| Some(acc * 2 % 11), true);
    let actual: Vec<usize> = producer.take(10).collect();
    assert_eq!(vec![1, 2, 4, 8, 5, 10, 9, 7, 3, 6], actual);

    let producer = produce(1, |acc| {
        let next = acc * 3;
        if next <= 100 {
            Some(next)
        } else {
            None
        }
    }, true);
    let actual: Vec<usize> = producer.collect();
    assert_eq!(vec![1, 3, 9, 27, 81], actual);

    let producer = produce(1, |acc| {
        let next = acc * 2;
        if next <= 100 {
            Some(next)
        } else {
            None
        }
    }, false);
    let actual: Vec<usize> = producer.collect();
    assert_eq!(vec![2, 4, 8, 16, 32, 64], actual);
}

#[test]
fn graph() {
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
}
