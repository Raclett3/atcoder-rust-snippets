use cargo_snippet::snippet;

#[snippet("bits")]
pub struct Bits(usize);

#[snippet("bits")]
impl Bits {
    pub fn at(&self, index: usize) -> bool {
        self.0 & 1 << index > 0
    }

    pub fn set_bits(&self) -> usize {
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
