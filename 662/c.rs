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

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        let n = scan.u();
        let a: Vec<usize> = scan.vec(n);
        let mut color_to_count = vec![0usize; n];
        for &x in &a {
            color_to_count[x - 1] += 1;
        }
        let count_max = *color_to_count.iter().max().unwrap();
        let kind = color_to_count.iter().filter(|&&x| x == count_max).count();

        /* (ans+1)*(count_max-1)+kind <= n
           ans <= (n-kind)/(count_max-1)-1
        */
        let ans = (n - kind) / (count_max - 1) - 1;

        writeln!(out, "{} ", ans).ok();
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
