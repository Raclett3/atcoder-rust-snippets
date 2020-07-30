use cargo_snippet::snippet;

#[snippet("modint")]
pub const MOD: usize = 1000000007;

#[snippet("modint")]
#[derive(Copy, Clone, Eq, PartialEq, std::fmt::Debug)]
pub struct ModInt(pub usize);

#[snippet("modint")]
impl std::ops::Add for ModInt {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        ModInt((self.0 + rhs.0) % MOD)
    }
}

#[snippet("modint")]
#[allow(clippy::suspicious_op_assign_impl)]
impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.0 %= MOD;
    }
}

#[snippet("modint")]
impl std::ops::Sub for ModInt {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        ModInt((self.0 + MOD - rhs.0) % MOD)
    }
}

#[snippet("modint")]
#[allow(clippy::suspicious_op_assign_impl)]
impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, rhs: Self) {
        self.0 += MOD - rhs.0;
        self.0 %= MOD;
    }
}

#[snippet("modint")]
impl std::ops::Mul for ModInt {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        ModInt((self.0 * rhs.0) % MOD)
    }
}

#[snippet("modint")]
#[allow(clippy::suspicious_op_assign_impl)]
impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
        self.0 %= MOD;
    }
}

#[snippet("modint")]
impl std::ops::Div for ModInt {
    type Output = Self;

    fn div(self, rhs: Self) -> Self {
        ModInt((self.0 * rhs.pow(MOD - 2).0) % MOD)
    }
}

#[snippet("modint")]
#[allow(clippy::suspicious_op_assign_impl)]
impl std::ops::DivAssign for ModInt {
    fn div_assign(&mut self, rhs: Self) {
        self.0 *= rhs.pow(MOD - 2).0;
        self.0 %= MOD;
    }
}

#[snippet("modint")]
impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0.to_string())
    }
}

#[snippet("modint")]
impl ModInt {
    pub fn pow(&self, power: usize) -> ModInt {
        let mut acc_base = *self;
        let mut acc_pow = power;
        let mut res = ModInt(1);
        while acc_pow > 0 {
            if acc_pow & 1 == 1 {
                res *= acc_base;
            }
            acc_base *= acc_base;
            acc_pow >>= 1;
        }
        res
    }
}

#[snippet("modint")]
pub struct ModIntFact {
    memo: Vec<ModInt>,
    memo_inv: Vec<ModInt>,
}

#[snippet("modint")]
impl ModIntFact {
    pub fn new(size: usize) -> Self {
        let mut memo = vec![ModInt(0); size + 1];
        memo[0] = ModInt(1);
        for n in 1..=size {
            memo[n] = memo[n - 1] * ModInt(n);
        }
        let memo_inv = memo.iter().map(|x| x.pow(MOD - 2)).collect();
        Self { memo, memo_inv }
    }

    pub fn fact(&self, n: usize) -> ModInt {
        self.memo[n]
    }

    pub fn fact_inv(&self, n: usize) -> ModInt {
        self.memo_inv[n]
    }

    pub fn ncr(&self, n: usize, r: usize) -> ModInt {
        self.memo[n] * self.memo_inv[r] * self.memo_inv[n - r]
    }

    pub fn npr(&self, n: usize, r: usize) -> ModInt {
        self.memo[n] * self.memo_inv[n - r]
    }

    pub fn nhr(&self, n: usize, r: usize) -> ModInt {
        self.memo[n + r - 1] * self.memo_inv[r] * self.memo_inv[n - 1]
    }
}

#[snippet("modint")]
impl std::iter::Sum for ModInt {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt(0), |x, y| x + y)
    }
}
