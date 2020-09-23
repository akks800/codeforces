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

fn kxywh(k: usize, x: usize, y: usize, w: usize, h: usize) -> Option<(usize, usize)> {
    const D: [(isize, isize); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    let xx = x as isize + D[k].0;
    let yy = y as isize + D[k].1;
    if 0 <= xx && xx < w as isize && 0 <= yy && yy < h as isize {
        Some((xx as usize, yy as usize))
    } else {
        None
    }
}

#[derive(Clone, Debug, Default)]
struct Solver {}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        let n: usize = scan.cin();
        let m: usize = scan.cin();

        let mut v = Vec::with_capacity(n);
        for _ in 0..n {
            v.push(scan.chars());
        }

        let mut dist: Vec<Vec<usize>> = vec![vec![0; m]; n];
        let mut q = VecDeque::new();

        for i in 0..n {
            for j in 0..m {
                if i == 0
                    || j == 0
                    || i == n - 1
                    || j == m - 1
                    || v[i][j] != v[i + 1][j]
                    || v[i][j] != v[i - 1][j]
                    || v[i][j] != v[i][j + 1]
                    || v[i][j] != v[i][j - 1]
                {
                    dist[i][j] = 1;
                    q.push_back((i, j));
                }
            }
        }

        while let Some((i, j)) = q.pop_front() {
            for k in 0..4 {
                if let Some((ii, jj)) = kxywh(k, i, j, n, m) {
                    if dist[ii][jj] == 0 {
                        dist[ii][jj] = dist[i][j] + 1;
                        q.push_back((ii, jj));
                    }
                }
            }
        }

        let mut ans = 0u64;
        for i in 0..n {
            for j in 0..m {
                ans += dist[i][j] as u64;
            }
        }

        writeln!(out, "{}", ans).ok();
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
