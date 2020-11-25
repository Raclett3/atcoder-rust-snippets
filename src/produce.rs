use cargo_snippet::snippet;

#[snippet("produce")]
pub struct Produce<T: Copy, F: Fn(T) -> Option<T>> {
    acc: Option<T>,
    func: F,
    include_init: bool,
}

#[snippet("produce")]
pub fn produce<T: Copy, F: Fn(T) -> Option<T>>(
    init: T,
    func: F,
    include_init: bool,
) -> Produce<T, F> {
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
                let current = if self.include_init { acc } else { next };
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

#[test]
fn test_produce() {
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
