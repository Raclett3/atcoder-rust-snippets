use cargo_snippet::snippet;

#[snippet("rollhash")]
struct RNG(u64);

#[snippet("rollhash")]
impl Iterator for RNG {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let RNG(mut x) = self;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        *self = RNG(x);
        Some(x)
    }
}

#[snippet("rollhash")]
const RHMOD: u64 = (1 << 61) - 1;

#[snippet("rollhash")]
type Hash = (u64, u64, u64, u64, u64, u64, u64, u64);

#[snippet("rollhash")]
fn u64_madd_mod(a: u64, b: u64, c: u64) -> u64 {
    ((a as u128 + b as u128 * c as u128) % RHMOD as u128) as u64
}

#[snippet("rollhash")]
fn u64_msub_mod(a: u64, b: u64, c: u64) -> u64 {
    ((a as u128 + RHMOD as u128 - (b as u128 * c as u128) % RHMOD as u128) % RHMOD as u128) as u64
}

#[snippet("rollhash")]
struct RHashFactory {
    base_nums: Vec<u64>,
}

#[snippet("rollhash")]
impl RHashFactory {
    fn new() -> Self {
        use std::time::{SystemTime, UNIX_EPOCH};
        let now = SystemTime::now();
        let time = now.duration_since(UNIX_EPOCH).unwrap().as_millis() as u64;
        let rng = RNG(time);
        let base_nums = rng.take(8).map(|x| x % (RHMOD - 3) + 2).collect();

        RHashFactory { base_nums }
    }

    fn instance(&self) -> RHash {
        RHash {
            hash: vec![0; 8],
            base_nums: &self.base_nums,
        }
    }

    fn into_acced(self, string: &str) -> AccedRHash {
        let hash = string
            .bytes()
            .chain("\0".bytes())
            .scan(self.instance(), |acc_hash, x| {
                let hash = acc_hash.hash();
                acc_hash.apply(x);
                Some(hash)
            })
            .collect();
        let power = (0..=string.len())
            .scan(vec![1; self.base_nums.len()], |acc, _| {
                let result = acc.clone();
                acc.iter_mut()
                    .zip(self.base_nums.iter())
                    .for_each(|(acc, &x)| *acc = u64_madd_mod(0, *acc, x));
                Some(result)
            })
            .collect();
            AccedRHash {
            base_nums_power: power,
            acced_hash: hash,
        }
    }
}

#[snippet("rollhash")]
struct AccedRHash {
    base_nums_power: Vec<Vec<u64>>,
    acced_hash: Vec<Hash>,
}

#[snippet("rollhash")]
impl AccedRHash {
    fn hash(&self, begin: usize, end: usize) -> Hash {
        let l_hash = self.acced_hash[begin];
        let r_hash = self.acced_hash[end];
        (
            u64_msub_mod(r_hash.0, l_hash.0, self.base_nums_power[end - begin][0]),
            u64_msub_mod(r_hash.1, l_hash.1, self.base_nums_power[end - begin][1]),
            u64_msub_mod(r_hash.2, l_hash.2, self.base_nums_power[end - begin][2]),
            u64_msub_mod(r_hash.3, l_hash.3, self.base_nums_power[end - begin][3]),
            u64_msub_mod(r_hash.4, l_hash.4, self.base_nums_power[end - begin][4]),
            u64_msub_mod(r_hash.5, l_hash.5, self.base_nums_power[end - begin][5]),
            u64_msub_mod(r_hash.6, l_hash.6, self.base_nums_power[end - begin][6]),
            u64_msub_mod(r_hash.7, l_hash.7, self.base_nums_power[end - begin][7]),
        )
    }
}

#[snippet("rollhash")]
struct RHash<'a> {
    hash: Vec<u64>,
    base_nums: &'a [u64],
}

#[snippet("rollhash")]
impl RHash<'_> {
    fn apply(&mut self, ch: u8) {
        for (h, &b) in self.hash.iter_mut().zip(self.base_nums.iter()) {
            *h = u64_madd_mod(ch as u64, *h, b);
        }
    }

    fn hash(&self) -> Hash {
        (
            self.hash[0],
            self.hash[1],
            self.hash[2],
            self.hash[3],
            self.hash[4],
            self.hash[5],
            self.hash[6],
            self.hash[7],
        )
    }
}

#[test]
fn test_rolling_hash() {
    let strs = ["a", "ab", "abab", "abcde", "ba", "abcdefghijklmnopqrstu"];
    let factory = RHashFactory::new();

    let hash_null = factory.instance();

    for i in 0..strs.len() {
        for j in (i + 1)..strs.len() {
            let mut hash_a_1 = factory.instance();
            let mut hash_a_2 = factory.instance();
            let mut hash_b = factory.instance();
            for b in strs[i].bytes() {
                hash_a_1.apply(b);
                hash_a_2.apply(b);
            }
            for b in strs[j].bytes() {
                hash_b.apply(b);
            }

            eprintln!("{} {}", strs[i], strs[j]);
            assert_eq!(hash_a_1.hash(), hash_a_2.hash());
            assert_ne!(hash_a_1.hash(), hash_b.hash());
            assert_ne!(hash_a_1.hash(), hash_null.hash());
        }
    }

    let acced = RHashFactory::new().into_acced("abcdefgabcdefg");
    assert_eq!(acced.hash(0, 3), acced.hash(0, 3));
    assert_eq!(acced.hash(0, 3), acced.hash(7, 10));
    assert_eq!(acced.hash(1, 4), acced.hash(8, 11));
    assert_ne!(acced.hash(0, 3), acced.hash(0, 4));
    assert_ne!(acced.hash(0, 3), acced.hash(7, 11));
    assert_ne!(acced.hash(0, 3), acced.hash(8, 11));
    assert_ne!(acced.hash(1, 4), acced.hash(7, 10));
}
