use cargo_snippet::snippet;

#[snippet("produce")]
pub struct Produce<T: Copy, F: Fn(T) -> Option<T>> {
    acc: Option<T>,
    func: F,
    include_init: bool,
}

#[snippet("produce")]
pub fn produce<T: Copy, F: Fn(T) -> Option<T>>(init: T, func: F, include_init: bool) -> Produce<T, F> {
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
