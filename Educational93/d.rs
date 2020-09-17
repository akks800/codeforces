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
    fn vecu(&mut self, n: usize) -> Vec<usize> {
        self.vec(n)
    }
}

///////

#[allow(unused_macros)]
macro_rules! chmin {
    ($dst:expr, $src:expr) => {{
        let b = $dst > $src;
        if b {
            $dst = $src;
        }
        b
    }};
}

#[allow(unused_macros)]
macro_rules! chmax {
    ($dst:expr, $src:expr) => {{
        let b = $dst < $src;
        if b {
            $dst = $src;
        }
        b
    }};
}



#[derive(Clone, Debug, Default)]
struct Solver {}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        let nr: usize = scan.cin();
        let ng: usize = scan.cin();
        let nb: usize = scan.cin();

        let mut r:Vec<u64> = scan.vec(nr);
        let mut g:Vec<u64> = scan.vec(ng);
        let mut b:Vec<u64> = scan.vec(nb);

        r.sort_unstable_by_key(|x|3000-x);
        g.sort_unstable_by_key(|x|3000-x);
        b.sort_unstable_by_key(|x|3000-x);
        r.push(0);
        g.push(0);
        b.push(0);

        let mut dp = vec![vec![vec![0;nb+2];ng+2];nr+2];
        for i in 0..=nr {
            for j in 0..=ng {
                for k in 0..=nb {
                    chmax!(dp[i+1][j+1][k], dp[i][j][k]+r[i]*g[j]);
                    chmax!(dp[i][j+1][k+1], dp[i][j][k]+g[j]*b[k]);
                    chmax!(dp[i+1][j][k+1], dp[i][j][k]+b[k]*r[i]);
                }
            }
        }

        let mut ans = 0;
        for i in 0..=nr {
            for j in 0..=ng {
                for k in 0..=nb {
                    chmax!(ans, dp[i][j][k]);
                }
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
