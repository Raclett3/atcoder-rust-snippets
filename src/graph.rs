use cargo_snippet::snippet;

#[snippet("graph")]
use std::cmp::*;
#[snippet("graph")]
use std::collections::binary_heap::BinaryHeap;

#[snippet("graph")]
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct GraphEdge {
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
    pub fn new(node_to: usize, cost: isize) -> Self {
        Self { node_to, cost }
    }
}

#[snippet("graph")]
pub struct Graph {
    pub edges: Vec<Vec<GraphEdge>>,
    pub nodes: usize,
}

#[snippet("graph")]
impl Graph {
    pub fn new(nodes: usize) -> Self {
        Self {
            edges: vec![vec![]; nodes],
            nodes,
        }
    }

    pub fn edge_undirected(&mut self, node_a: usize, node_b: usize) {
        self.edges[node_a].push(GraphEdge::new(node_b, 1));
        self.edges[node_b].push(GraphEdge::new(node_a, 1));
    }

    pub fn edge_directed(&mut self, node_from: usize, node_to: usize) {
        self.edges[node_from].push(GraphEdge::new(node_to, 1));
    }

    pub fn edge_undirected_costed(&mut self, node_a: usize, node_b: usize, cost: isize) {
        self.edges[node_a].push(GraphEdge::new(node_b, cost));
        self.edges[node_b].push(GraphEdge::new(node_a, cost));
    }

    pub fn edge_directed_costed(&mut self, node_from: usize, node_to: usize, cost: isize) {
        self.edges[node_from].push(GraphEdge::new(node_to, cost));
    }

    pub fn dijkstra(&self, node_from: usize) -> Vec<isize> {
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

    pub fn warshall_floyd(&self) -> Vec<Vec<isize>> {
        let mut costs = vec![vec![std::isize::MAX; self.nodes]; self.nodes];
        for i in 0..self.nodes {
            costs[i][i] = 0;
        }
        for (node_from, edges) in self.edges.iter().enumerate() {
            for GraphEdge {node_to, cost} in edges.iter() {
                costs[node_from][*node_to] = *cost;
            }
        }
        for k in 0..self.nodes {
            for i in 0..self.nodes {
                for j in 0..self.nodes {
                    costs[i][j] = std::cmp::min(costs[i][j], costs[i][k].saturating_add(costs[k][j]));
                }
            }
        }
        costs
    }
}
