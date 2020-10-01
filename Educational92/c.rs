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

#[derive(Clone, Debug, Default)]
struct Solver {}

fn f(i: usize, j: usize, t: &[usize]) -> usize {
    let mut ans = 0;
    for pos in 0..t.len() {
        if ans & 1 == 0 {
            if t[pos] == i {
                ans += 1;
            }
        } else {
            if t[pos] == j {
                ans += 1;
            }
        }
    }
    if i == j {
        ans        
    } else {
        ans / 2 * 2
    }
}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        // 1389c
        let t: Vec<usize> = scan
            .chars()
            .iter()
            .map(|&c| c as usize - '0' as usize)
            .collect();

        let mut good_chars = 0;
        for i in 0..=9 {
            for j in 0..=9 {
                chmax!(good_chars, f(i, j, &t));
            }
        }

        writeln!(out, "{}", t.len() - good_chars).ok();
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
