//use std::cmp::*;
use std::collections::*;
use std::io::*;
use std::str::*;

// scanner from https://codeforces.com/contest/1396/submission/91365784
struct Scanner {
    stdin: Stdin,
    buffer: VecDeque<String>,
}
#[allow(dead_code)]
impl Scanner {
    fn new() -> Self {
        Scanner {
            stdin: stdin(),
            buffer: VecDeque::new(),
        }
    }
    fn cin<T: FromStr>(&mut self) -> T {
        while self.buffer.is_empty() {
            let mut line = String::new();
            let _ = self.stdin.read_line(&mut line);
            for w in line.split_whitespace() {
                self.buffer.push_back(String::from(w));
            }
        }
        self.buffer.pop_front().unwrap().parse::<T>().ok().unwrap()
    }
    fn chars(&mut self) -> Vec<char> {
        self.cin::<String>().chars().collect()
    }
    fn vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
        (0..n).map(|_| self.cin()).collect()
    }
    fn u(&mut self) -> usize {
        self.cin()
    }
    fn vecu(&mut self, n: usize) -> Vec<usize> {
        self.vec(n)
    }
}

//
// ModInt
//

const M: i64 = 998244353;
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct ModInt {
    val: i64,
}
#[allow(dead_code)]
const M0: ModInt = ModInt { val: 0 };
const M1: ModInt = ModInt { val: 1 };
//const M_ERROR: ModInt = ModInt { val: -1 };

impl ModInt {
    fn new(v: i64) -> ModInt {
        ModInt {
            val: ModInt::val_from_i64(v),
        }
    }
    fn val_from_i64(v: i64) -> i64 {
        let t = v % M;
        if t < 0 {
            t + M
        } else {
            t
        }
    }
    fn val_from_addval(t: i64) -> i64 {
        if t >= M {
            t - M
        } else {
            t
        }
    }

    fn pow(self, exp: i64) -> ModInt {
        if exp == 0 {
            M1
        } else {
            let x = self.pow(exp / 2);
            if exp % 2 == 0 {
                x * x
            } else {
                self * x * x
            }
        }
    }

    fn inv(self) -> ModInt {
        self.pow(M - 2)
    }
}

impl std::ops::Add for ModInt {
    type Output = ModInt;
    fn add(self, rhs: ModInt) -> ModInt {
        ModInt {
            val: ModInt::val_from_addval(self.val + rhs.val),
        }
    }
}
impl std::ops::Add<i64> for ModInt {
    type Output = ModInt;
    fn add(self, rhs: i64) -> ModInt {
        ModInt {
            val: ModInt::val_from_i64(self.val + rhs),
        }
    }
}
impl std::ops::AddAssign for ModInt {
    fn add_assign(&mut self, other: ModInt) {
        self.val = ModInt::val_from_addval(self.val + other.val);
    }
}
impl std::ops::AddAssign<i64> for ModInt {
    fn add_assign(&mut self, other: i64) {
        self.val = ModInt::val_from_i64(self.val + other);
    }
}

impl std::ops::Sub for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: ModInt) -> ModInt {
        ModInt {
            val: ModInt::val_from_addval(self.val - rhs.val + M),
        }
    }
}
impl std::ops::Sub<i64> for ModInt {
    type Output = ModInt;
    fn sub(self, rhs: i64) -> ModInt {
        self - ModInt::new(rhs)
    }
}
impl std::ops::SubAssign for ModInt {
    fn sub_assign(&mut self, other: ModInt) {
        self.val = ModInt::val_from_addval(self.val - other.val + M);
    }
}
impl std::ops::SubAssign<i64> for ModInt {
    fn sub_assign(&mut self, other: i64) {
        self.val = ModInt::val_from_i64(self.val - other);
    }
}

impl std::ops::Neg for ModInt {
    type Output = ModInt;
    fn neg(self) -> ModInt {
        ModInt {
            val: if self.val == 0 { 0 } else { M - self.val },
        }
    }
}

impl std::ops::Mul for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: ModInt) -> ModInt {
        ModInt {
            val: self.val * rhs.val % M,
        }
    }
}
impl std::ops::Mul<i64> for ModInt {
    type Output = ModInt;
    fn mul(self, rhs: i64) -> ModInt {
        self * ModInt::new(rhs)
    }
}
impl std::ops::MulAssign for ModInt {
    fn mul_assign(&mut self, other: ModInt) {
        self.val = self.val * other.val % M;
    }
}
impl std::ops::MulAssign<i64> for ModInt {
    fn mul_assign(&mut self, other: i64) {
        self.val = self.val * (other % M) % M;
    }
}

impl std::ops::Div for ModInt {
    type Output = ModInt;
    fn div(self, rhs: ModInt) -> ModInt {
        self * rhs.inv()
    }
}
impl std::ops::Div<i64> for ModInt {
    type Output = ModInt;
    fn div(self, rhs: i64) -> ModInt {
        self / ModInt::new(rhs)
    }
}

impl std::fmt::Display for ModInt {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // '
        write!(f, "{}", self.val)
    }
}

//
// nCr
//
#[derive(Debug, Clone)]
struct Comb {
    fac: Vec<ModInt>,
    finv: Vec<ModInt>,
    inv: Vec<ModInt>,
}

impl Comb {
    fn new(size: usize) -> Comb {
        let mut fac = vec![M1; size];
        let mut finv = vec![M1; size];
        let mut inv = vec![M1; size];

        for i in 2..size {
            fac[i] = fac[i - 1] * (i as i64);
            inv[i] = -inv[M as usize % i] * (M / (i as i64));
            finv[i] = finv[i - 1] * inv[i];
        }

        return Comb {
            fac: fac,
            finv: finv,
            inv: inv,
        };
    }
}
trait CombC<T> {
    fn c(&self, n: T, k: T) -> ModInt;
    fn p(&self, n: T, k: T) -> ModInt;
}

impl CombC<usize> for Comb {
    fn c(&self, n: usize, k: usize) -> ModInt {
        if n < k {
            M0
        } else {
            self.fac[n] * self.finv[k] * self.finv[n - k]
        }
    }
    fn p(&self, n: usize, k: usize) -> ModInt {
        if n < k {
            M0
        } else {
            self.fac[n] * self.finv[n - k]
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Solver {}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        // 1420d
        let n = scan.u();
        let k = scan.u();
        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            let l: u64 = scan.cin();
            let r: u64 = scan.cin();
            v.push((l * 2, 1i32));
            v.push((r * 2 + 1, -1));
        }
        v.sort_unstable_by_key(|&x| x.0);

        let c = Comb::new(n + 5);
        let mut on = 0;
        let mut ans = M0;
        for &(_, ud) in &v {
            on += ud;
            if ud == 1 {
                ans += c.c(on as usize - 1, k as usize - 1);
            }
        }

        writeln!(out, "{} ", ans).ok();
    }
}

fn solve() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let testcase = 1;
    //let testcase: usize = scan.cin();
    for _ in 0..testcase {
        let mut s: Solver = Default::default();
        s.solve(&mut scan, out);
    }
}

fn main() {
    let th = std::thread::Builder::new().stack_size(64 * 1024 * 1024);
    th.spawn(|| solve()).unwrap().join().unwrap()
}
