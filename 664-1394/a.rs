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

fn f(h_day: usize, l_day: usize, acch: &Vec<i64>, accl: &Vec<i64>) -> i64 {
    acch[h_day] + accl[l_day]
}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        let n = scan.u();
        let d = scan.u();
        let m = scan.cin();
        let a: Vec<i64> = scan.vec(n);

        let mut ah: Vec<i64> = a.iter().cloned().filter(|&x| x > m).collect();
        let mut al: Vec<i64> = a.iter().cloned().filter(|&x| x <= m).collect();
        ah.sort_unstable_by_key(|x| 1000_000_000 - x);
        al.sort_unstable_by_key(|x| 1000_000_000 - x);

        let mut sum = 0;
        let mut acch: Vec<i64> = ah
            .iter()
            .map(|&x| {
                sum += x;
                sum - x
            })
            .collect();
        acch.push(sum);
        sum = 0;
        let mut accl: Vec<i64> = al
            .iter()
            .map(|&x| {
                sum += x;
                sum - x
            })
            .collect();
        accl.push(sum);

        let ans2 = self.solve2(n, d, ah.len(), &acch, &accl);
        writeln!(out, "{} ", ans2).ok();
    }

    fn solve2(&self, n: usize, d: usize, hlen: usize, acch: &Vec<i64>, accl: &Vec<i64>) -> i64 {
        let mut ans = 0i64;
        if hlen == 0 {
            ans = f(0, n, &acch, &accl);
        } else {
            for h_day in 1..=hlen {
                let mut muzzle_day = (h_day - 1) * d;
                if h_day + muzzle_day < hlen {
                    muzzle_day = hlen - h_day;
                }
                if h_day + muzzle_day > n {
                    break;
                }
                let l_day = n - h_day - muzzle_day;
                chmax!(ans, f(h_day, l_day, &acch, &accl));
            }
        }
        ans
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