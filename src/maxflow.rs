use cargo_snippet::snippet;

#[snippet("maxflow")]
#[derive(Copy, Clone, Debug, PartialEq)]
struct Edge {
    from: usize,
    to: usize,
    flow: isize,
    capacity: isize,
}

#[snippet("maxflow")]
impl Edge {
    fn new(from: usize, to: usize, flow: isize, capacity: isize) -> Self {
        Edge {
            from,
            to,
            flow,
            capacity,
        }
    }
}

#[snippet("maxflow")]
#[derive(Copy, Clone, Debug, PartialEq)]
struct EdgeInternal {
    to: usize,
    capacity: isize,
    rev_ref: usize,
}

#[snippet("maxflow")]
impl EdgeInternal {
    fn new(to: usize, capacity: isize, rev_ref: usize) -> Self {
        EdgeInternal {
            to,
            capacity,
            rev_ref,
        }
    }
}

#[snippet("maxflow")]
struct MaxFlow {
    nodes: usize,
    edges: Vec<Vec<EdgeInternal>>,
    edge_indices: Vec<(usize, usize)>,
}

#[snippet("maxflow")]
fn fill<T: Clone>(slice: &mut [T], value: T) {
    for item in slice.iter_mut() {
        *item = value.clone();
    }
}

#[snippet("maxflow")]
impl MaxFlow {
    fn new(nodes: usize) -> Self {
        MaxFlow {
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

        Edge::new(from, to, flow, capacity)
    }

    fn add_edge(&mut self, from: usize, to: usize, capacity: isize) {
        assert!(from < self.nodes);
        assert!(to < self.nodes);

        let edge_ref = self.edges[from].len();
        let edge_rev_ref = self.edges[to].len();
        self.edge_indices.push((from, edge_ref));

        self.edges[from].push(EdgeInternal::new(to, capacity, edge_rev_ref));
        self.edges[to].push(EdgeInternal::new(from, 0, edge_ref));
    }

    fn edges(&self) -> Vec<Edge> {
        self.edge_indices
            .iter()
            .map(|&idx| self.to_external_edge(idx))
            .collect()
    }

    fn levels(&self, levels: &mut [Option<usize>], node: usize) {
        fill(levels, None);

        let mut queue = std::collections::VecDeque::new();
        queue.push_back((node, 0));

        while let Some((node, dist)) = queue.pop_front() {
            if levels[node].is_some() {
                continue;
            }

            levels[node] = Some(dist);

            for edge in self.edges[node].iter().filter(|e| e.capacity > 0) {
                queue.push_back((edge.to, dist + 1));
            }
        }
    }

    fn flow(
        &mut self,
        node: usize,
        sink: usize,
        limit: isize,
        checked: &mut [usize],
        levels: &[Option<usize>],
    ) -> isize {
        if node == sink || limit == 0 {
            return limit;
        }

        let mut flow = 0;

        for i in checked[node].. {
            let edge = if let Some(edge) = self.edges[node].get(i) {
                *edge
            } else {
                break;
            };
            checked[node] += 1;

            let dest_is_far = levels[node].and_then(|from| levels[edge.to].map(|to| from < to));

            if !dest_is_far.unwrap_or(false) {
                continue;
            }

            let f = self.flow(
                edge.to,
                sink,
                isize::min(limit - flow, edge.capacity),
                checked,
                levels,
            );

            self.edges[node][i].capacity -= f;
            self.edges[edge.to][edge.rev_ref].capacity += f;
            flow += f;
        }

        flow
    }

    fn max_flow_limited(&mut self, source: usize, sink: usize, limit: isize) -> isize {
        let mut flow = 0;
        let mut levels = vec![None; self.nodes];
        let mut checked = vec![0; self.nodes];

        loop {
            self.levels(&mut levels, source);

            if levels[sink].is_none() {
                break flow;
            }

            fill(&mut checked, 0);

            loop {
                let f = self.flow(source, sink, limit, &mut checked, &levels);
                if f <= 0 {
                    break;
                }

                flow += f;
            }
        }
    }

    fn max_flow(&mut self, source: usize, sink: usize) -> isize {
        self.max_flow_limited(source, sink, std::isize::MAX)
    }

    fn min_cut(&self, s: usize) -> Vec<bool> {
        let mut visited = vec![false; self.nodes];
        let mut queue = std::collections::VecDeque::new();
        queue.push_back(s);
        visited[s] = true;

        while let Some(node) = queue.pop_front() {
            for edge in &self.edges[node] {
                if visited[edge.to] || edge.capacity <= 0 {
                    continue;
                }

                queue.push_back(edge.to);
                visited[edge.to] = true;
            }
        }

        visited
    }
}

#[test]
fn test_maxflow() {
    let edges = [
        (0, 1, 9),
        (0, 2, 3),
        (1, 2, 3),
        (1, 3, 4),
        (2, 3, 5),
        (2, 4, 4),
        (3, 4, 8),
    ];

    let mut graph = MaxFlow::new(5);

    for &(from, to, capacity) in &edges {
        graph.add_edge(from, to, capacity);
    }

    assert_eq!(graph.max_flow(0, 4), 10);
    let edges = graph.edges();
    let flow_from_source: isize = edges.iter().filter(|e| e.from == 0).map(|e| e.flow).sum();
    assert_eq!(flow_from_source, 10);
    let flow_to_sink: isize = edges.iter().filter(|e| e.to == 4).map(|e| e.flow).sum();
    assert_eq!(flow_to_sink, 10);
    let min_cut = graph.min_cut(0);
    let min_cut_flow: isize = edges
        .iter()
        .filter(|e| min_cut[e.from] && !min_cut[e.to])
        .map(|e| e.flow)
        .sum();
    assert_eq!(min_cut_flow, 10);
}
