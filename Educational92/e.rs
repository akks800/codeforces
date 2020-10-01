use std::cmp::*;
use std::collections::*;
use std::io::*;
use std::ops::*;
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
    fn uu(&mut self) -> u64 {
        self.cin()
    }
    fn vecu(&mut self, n: usize) -> Vec<usize> {
        self.vec(n)
    }
    fn vecll(&mut self, n: usize) -> Vec<i64> {
        self.vec(n)
    }
}

#[allow(unused_macros)]
macro_rules! chmax {
    ($dst:expr, $src:expr) => {{
        let src = $src;
        let dst = &mut $dst;
        let b = *dst < src;
        if b {
            *dst = src;
        }
        b
    }};
}

fn gcd<T>(a: T, b: T) -> T
where
    T: Copy + Sub<Output = T> + Mul<Output = T> + Div<Output = T> + PartialOrd + Default,
{
    if a < b {
        gcd(b, a)
    } else {
        let k = a / b;
        if a - k * b == Default::default() {
            b
        } else {
            gcd(b, a - k * b)
        }
    }
}

#[derive(Clone, Debug, Default)]
struct Solver {}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        // 1389e
        let m = scan.uu();
        let d = scan.uu();
        let w = scan.uu();

        /*
            (x-1)*d+(y-1) % w ==  (y-1)*d+(x-1) % w
            1 <= x <= d
            1 <= y <= d
            1 <= x <= m
            1 <= y <= m
            x < y

        x-1 = xx
        y-1 = yy

        (xx*d+yy - yy*d+xx) %w == 0
        (d-1)(yy-xx) % w == 0
        0 <= xx < d
        0 <= yy < d
        0 <= xx < m
        0 <= yy < m
        xx < yy

        */
        let u = if d == 1 { 1 } else { w / gcd(d - 1, w) };
        let md = min(d, m);
        /*
         0 <= xx < md
         0 <= yy < md
         (yy-xx) % u == 0
         xx < yy

         yy-xx=u -> md-u個
         yy-xx=2u -> md-2u個
         ans = (md-u) + (md-2u) + (md-3u) + ... + (md-ku), k=md/u

        */
        let k = md / u;
        let ans = md * k - u * k * (k + 1) / 2;

        writeln!(out, "{}", ans).ok();
    }
}

fn solve() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    //let testcase = 1;
    let testcase: usize = scan.cin();
    for _ in 0..testcase {
        let mut s: Solver = Default::default();
        s.solve(&mut scan, out);
    }
}

fn main() {
    let th = std::thread::Builder::new().stack_size(64 * 1024 * 1024);
    th.spawn(|| solve()).unwrap().join().unwrap()
}
