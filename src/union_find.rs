use cargo_snippet::snippet;

#[snippet("union_find")]
pub struct UnionFind {
    root: Vec<usize>,
}

#[snippet("union_find")]
impl UnionFind {
    pub fn new(nodes: usize) -> Self {
        Self {
            root: (0..=nodes).collect(),
        }
    }

    pub fn merge(&mut self, a: usize, b: usize) {
        self.root[a] = self.root[b];
    }

    pub fn same_root(&mut self, a: usize, b: usize) -> bool {
        self.root_of(a) == self.root_of(b)
    }

    pub fn root_of(&mut self, a: usize) -> usize {
        if self.root[a] == a {
            return a;
        }
        self.root[a] = self.root_of(self.root[a]);
        self.root[a]
    }
}
