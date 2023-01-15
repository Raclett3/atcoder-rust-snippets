use cargo_snippet::snippet;

#[snippet("mincostflow")]
#[derive(Copy, Clone, Debug, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
    flow: isize,
    capacity: isize,
    cost: isize,
}

#[snippet("mincostflow")]
impl Edge {
    fn new(from: usize, to: usize, flow: isize, capacity: isize, cost: isize) -> Self {
        Edge {
            from,
            to,
            flow,
            capacity,
            cost,
        }
    }
}

#[snippet("mincostflow")]
#[derive(Copy, Clone, Debug, PartialEq)]
struct EdgeInternal {
    to: usize,
    capacity: isize,
    cost: isize,
    rev_ref: usize,
}

#[snippet("mincostflow")]
impl EdgeInternal {
    fn new(to: usize, capacity: isize, cost: isize, rev_ref: usize) -> Self {
        EdgeInternal {
            to,
            capacity,
            cost,
            rev_ref,
        }
    }
}

#[snippet("mincostflow")]
struct MinCostFlow {
    nodes: usize,
    edges: Vec<Vec<EdgeInternal>>,
    edge_indices: Vec<(usize, usize)>,
}

#[snippet("mincostflow")]
fn fill<T: Clone>(slice: &mut [T], value: T) {
    for item in slice.iter_mut() {
        *item = value.clone();
    }
}

#[snippet("mincostflow")]
impl MinCostFlow {
    fn new(nodes: usize) -> Self {
        MinCostFlow {
            nodes,
            edges: vec![Vec::new(); nodes],
            edge_indices: Vec::new(),
        }
    }

    fn to_external_edge(&self, edge_index: (usize, usize)) -> Edge {
        let edge = &self.edges[edge_index.0][edge_index.1];
        let edge_rev = &self.edges[edge.to][edge.rev_ref];

        let to = edge.to;
        let from = edge_rev.to;
        let flow = edge_rev.capacity;
        let capacity = edge.capacity + edge_rev.capacity;
        let cost = edge.cost;

        Edge::new(from, to, flow, capacity, cost)
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: isize, cost: isize) {
        assert!(from < self.nodes);
        assert!(to < self.nodes);

        let edge_ref = self.edges[from].len();
        let edge_rev_ref = self.edges[to].len();
        self.edge_indices.push((from, edge_ref));

        self.edges[from].push(EdgeInternal::new(to, capacity, cost, edge_rev_ref));
        self.edges[to].push(EdgeInternal::new(from, 0, -cost, edge_ref));
    }

    fn edges(&self) -> Vec<Edge> {
        self.edge_indices
            .iter()
            .map(|&idx| self.to_external_edge(idx))
            .collect()
    }

    fn path(&self, potential: &mut [isize], path: &mut [Option<(usize, usize)>], node: usize) {
        fill(path, None);

        let mut heap = std::collections::BinaryHeap::new();
        heap.push(std::cmp::Reverse((0, node, None)));

        while let Some(std::cmp::Reverse((dist, node, edge))) = heap.pop() {
            if path[node].is_some() {
                continue;
            }

            path[node] = edge;

            let edges = self.edges[node]
                .iter()
                .enumerate()
                .filter(|(_, e)| e.capacity > 0);

            for (i, edge) in edges {
                heap.push(std::cmp::Reverse((
                    dist + edge.cost + potential[node] - potential[edge.to],
                    edge.to,
                    Some((node, i)),
                )));
            }

            potential[node] += dist;
        }
    }

    fn flow(
        &mut self,
        source: usize,
        node_to: usize,
        limit: isize,
        path: &[Option<(usize, usize)>],
    ) -> Option<(isize, isize)> {
        if limit <= 0 {
            return None;
        }

        if node_to == source {
            return Some((limit, 0));
        }

        let (node_from, edge_index) = path[node_to]?;

        let limit = isize::min(limit, self.edges[node_from][edge_index].capacity);

        let (flow, cost) = self.flow(source, node_from, limit, path)?;

        let EdgeInternal {
            to,
            rev_ref,
            cost: cost_per_flow,
            ..
        } = self.edges[node_from][edge_index];
        self.edges[node_from][edge_index].capacity -= flow;
        self.edges[to][rev_ref].capacity += flow;

        Some((flow, cost + flow * cost_per_flow))
    }

    fn min_cost_max_flow_limited(
        &mut self,
        source: usize,
        sink: usize,
        limit: isize,
    ) -> (isize, isize) {
        let mut flow = 0;
        let mut cost = 0;
        let mut path = vec![None; self.nodes];
        let mut potential = vec![0; self.nodes];

        loop {
            self.path(&mut potential, &mut path, source);

            if let Some((f, c)) = self.flow(source, sink, limit - flow, &path) {
                flow += f;
                cost += c;
            } else {
                break;
            }
        }

        (flow, cost)
    }

    fn min_cost_max_flow(&mut self, source: usize, sink: usize) -> (isize, isize) {
        self.min_cost_max_flow_limited(source, sink, std::isize::MAX)
    }
}

#[test]
fn test_mincostflow() {
    let edges = [
        (0, 1, 8, 3),
        (0, 2, 5, 2),
        (1, 2, 9, 7),
        (1, 3, 4, 3),
        (2, 3, 3, 4),
        (2, 4, 9, 2),
        (3, 4, 12, 9),
        (3, 5, 9, 6),
        (4, 5, 2, 1),
    ];

    let mut graph = MinCostFlow::new(6);

    for &(from, to, capacity, cost) in &edges {
        graph.add_edge(from, to, capacity, cost);
    }

    let (flow, cost) = graph.min_cost_max_flow(0, 5);

    assert_eq!(flow, 9);
    assert_eq!(cost, 94);
    let edges = graph.edges();
    let flow_from_source: isize = edges.iter().filter(|e| e.from == 0).map(|e| e.flow).sum();
    assert_eq!(flow_from_source, 9);
    let flow_to_sink: isize = edges.iter().filter(|e| e.to == 5).map(|e| e.flow).sum();
    assert_eq!(flow_to_sink, 9);
    let cost_sum: isize = edges.iter().map(|e| e.flow * e.cost).sum();
    assert_eq!(cost_sum, 94);

    let edges = [
        (0, 1, 8, 3),
        (0, 2, 5, 2),
        (1, 2, 9, 7),
        (1, 3, 4, 3),
        (2, 3, 3, 4),
        (2, 4, 9, 2),
        (3, 4, 12, 9),
        (3, 5, 9, 6),
        (4, 5, 2, 1),
    ];

    let mut graph = MinCostFlow::new(6);

    for &(from, to, capacity, cost) in &edges {
        graph.add_edge(from, to, capacity, cost);
    }

    let (flow, cost) = graph.min_cost_max_flow_limited(0, 5, 6);

    assert_eq!(flow, 6);
    assert_eq!(cost, 58);
    let edges = graph.edges();
    let flow_from_source: isize = edges.iter().filter(|e| e.from == 0).map(|e| e.flow).sum();
    assert_eq!(flow_from_source, 6);
    let flow_to_sink: isize = edges.iter().filter(|e| e.to == 5).map(|e| e.flow).sum();
    assert_eq!(flow_to_sink, 6);
    let cost_sum: isize = edges.iter().map(|e| e.flow * e.cost).sum();
    assert_eq!(cost_sum, 58);

}
