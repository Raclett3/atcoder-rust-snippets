use cargo_snippet::snippet;

#[snippet("geometric")]
pub struct Geometric {
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
    pub fn new(init: isize, ratio: isize) -> Self {
        Self {
            current: init,
            ratio,
        }
    }
}
