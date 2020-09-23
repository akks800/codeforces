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

///////

#[allow(unused_macros)]
macro_rules! chmin {
    ($dst:expr, $src:expr) => {{
        let src = $src;
        let dst = &mut $dst;
        let b = *dst > src;
        if b {
            *dst = src;
        }
        b
    }};
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

#[derive(Clone, Debug, Default)]
struct Solver {}

fn factorization(mut n: u64) -> BTreeMap<u64, u64> {
    let mut m = BTreeMap::new();
    for i in 2.. {
        if i * i > n {
            if n != 1 {
                m.insert(n, 1);
            }
            break;
        }

        while n % i == 0 {
            m.entry(i).and_modify(|x| *x += 1).or_insert(1);
            n /= i;
        }
    }
    m
}

// 約数をグループ分け。どの素数で割れるかどうかで分ける。
fn dfs(
    i: usize,
    len: usize,
    v: &mut Vec<Vec<u64>>,
    primes: &Vec<u64>,
    pows: &Vec<u64>,
    mut mul: u64,
    group: usize,
) {
    if i == len {
        v[group].push(mul);
    } else {
        for j in 0..=pows[i] {
            let newgroup = group | if j == 0 { 0 } else { 1 << i };
            dfs(i + 1, len, v, primes, pows, mul, newgroup);
            mul *= primes[i];
        }
    }
}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        let n: u64 = scan.cin();
        let m = factorization(n);
        let primes: Vec<u64> = m.iter().map(|x| *x.0).collect();
        let pows: Vec<u64> = m.iter().map(|x| *x.1).collect();

        let len = m.len();
        let vlen = 1 << len;
        let mut v = vec![Vec::new(); vlen];
        dfs(0, len, &mut v, &primes, &pows, 1, 0);
        v[0].clear();

        let min_move;
        if len == 2 && v[vlen - 1].len() == 1 {
            min_move = 1;
        } else {
            let tmp = v[vlen - 1].pop().unwrap();
            v[0].push(tmp);
            min_move = 0;
        }

        // Gray code を使う
        for i in 0..vlen {
            let grey_code = i ^ (i >> 1);
            for &j in &v[grey_code] {
                write!(out, "{} ", j).ok();
            }
        }
        writeln!(out, "").ok();
        writeln!(out, "{}", min_move).ok();
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
