#![allow(clippy::needless_range_loop)]

use cargo_snippet::snippet;

#[snippet("graph")]
use std::cmp::*;
#[snippet("graph")]
use std::collections::binary_heap::BinaryHeap;

#[snippet("graph")]
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd)]
pub struct GraphEdge {
    cost: isize,
    node_to: usize,
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

    pub fn dijkstra(&self, node_from: usize) -> Vec<Option<isize>> {
        let mut costs = vec![None; self.nodes];
        let mut heap: BinaryHeap<Reverse<GraphEdge>> = BinaryHeap::new();
        heap.push(Reverse(GraphEdge::new(node_from, 0)));
        while let Some(Reverse(GraphEdge { cost, node_to })) = heap.pop() {
            if cost >= costs[node_to].unwrap_or(std::isize::MAX) {
                continue;
            }
            costs[node_to] = Some(cost);

            for edge in &self.edges[node_to] {
                let next = GraphEdge::new(edge.node_to, cost + edge.cost);
                heap.push(Reverse(next));
            }
        }
        costs
    }

    pub fn warshall_floyd(&self) -> Vec<Vec<Option<isize>>> {
        let mut costs = vec![vec![None; self.nodes]; self.nodes];
        for i in 0..self.nodes {
            costs[i][i] = Some(0);
        }
        for (node_from, edges) in self.edges.iter().enumerate() {
            for GraphEdge { node_to, cost } in edges.iter() {
                costs[node_from][*node_to] = Some(*cost);
            }
        }
        for k in 0..self.nodes {
            for i in 0..self.nodes {
                for j in 0..self.nodes {
                    costs[i][j] = match (costs[i][j], (costs[i][k], costs[k][j])) {
                        (Some(cost), (Some(cost_ik), Some(cost_kj))) => {
                            Some(cost.min(cost_ik + cost_kj))
                        }
                        (None, (Some(cost_ik), Some(cost_kj))) => Some(cost_ik + cost_kj),
                        (cost, _) => cost,
                    };
                }
            }
        }
        costs
    }
}

#[test]
fn test_graph() {
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
    assert_eq!(
        vec![
            Some(0),
            Some(6),
            Some(19),
            Some(9),
            Some(12),
            Some(12),
            Some(15),
            Some(2)
        ],
        graph.dijkstra(0)
    );
    let mut graph = Graph::new(6);
    graph.edge_undirected_costed(0, 1, 5);
    graph.edge_undirected_costed(0, 4, 15);
    graph.edge_undirected_costed(1, 2, 1);
    graph.edge_undirected_costed(2, 3, 2);
    graph.edge_undirected_costed(2, 4, 9);
    graph.edge_undirected_costed(3, 4, 6);
    let expected = vec![
        vec![Some(0), Some(5), Some(6), Some(8), Some(14), None],
        vec![Some(5), Some(0), Some(1), Some(3), Some(9), None],
        vec![Some(6), Some(1), Some(0), Some(2), Some(8), None],
        vec![Some(8), Some(3), Some(2), Some(0), Some(6), None],
        vec![Some(14), Some(9), Some(8), Some(6), Some(0), None],
        vec![None, None, None, None, None, Some(0)],
    ];
    assert_eq!(expected, graph.warshall_floyd());
}
