use cargo_snippet::snippet;

#[snippet("modint")]
pub const MOD: usize = 1000000007;

#[snippet("modint")]
pub fn mint(number: usize) -> ModInt {
    ModInt(number % MOD)
}

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
    size: usize,
}

#[snippet("modint")]
impl ModIntFact {
    pub fn new() -> Self {
        Self {
            memo: vec![ModInt(1)],
            memo_inv: vec![ModInt(1)],
            size: 0,
        }
    }

    pub fn extend(&mut self, size: usize) {
        if self.size >= size {
            return;
        }

        self.memo.resize(size + 1, ModInt(0));
        self.memo_inv.resize(size + 1, ModInt(0));

        for n in (self.size + 1)..=size {
            self.memo[n] = self.memo[n - 1] * ModInt(n);
            self.memo_inv[n] = self.memo[n].pow(MOD - 2);
        }

        self.size = size;
    }

    pub fn fact(&mut self, n: usize) -> ModInt {
        self.extend(n);
        self.memo[n]
    }

    pub fn fact_inv(&mut self, n: usize) -> ModInt {
        self.extend(n);
        self.memo_inv[n]
    }

    pub fn ncr(&mut self, n: usize, r: usize) -> ModInt {
        self.fact(n) * self.fact_inv(r) * self.fact_inv(n - r)
    }

    pub fn npr(&mut self, n: usize, r: usize) -> ModInt {
        self.fact(n) * self.fact_inv(n - r)
    }

    pub fn nhr(&mut self, n: usize, r: usize) -> ModInt {
        self.fact(n + r - 1) * self.fact_inv(r) * self.fact_inv(n - 1)
    }
}

#[snippet("modint")]
impl std::iter::Sum for ModInt {
    fn sum<I: Iterator<Item = Self>>(iter: I) -> Self {
        iter.fold(ModInt(0), |x, y| x + y)
    }
}
