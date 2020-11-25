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
        let a = self.root_of(a);
        let b = self.root_of(b);

        if a < b {
            self.root[a] = b;
        } else {
            self.root[b] = a;
        }
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

#[test]
fn test_union_find() {
    let mut uf = UnionFind::new(5);
    uf.merge(0, 2);
    uf.merge(2, 4);
    uf.merge(3, 4);
    assert!(uf.same_root(0, 2));
    assert!(uf.same_root(3, 4));
    assert!(uf.same_root(0, 4));
    assert!(!uf.same_root(0, 1));
    assert!(!uf.same_root(1, 4));
    assert!(uf.same_root(1, 1));
}
