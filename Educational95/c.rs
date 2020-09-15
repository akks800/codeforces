use std::cmp::*;
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
}

///////

#[derive(Clone, Debug, Default)]
struct Solver {}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        let n: usize = scan.cin();
        let mut a: Vec<usize> = scan.vec(n);

        a.push(0);
        a.push(0);
        // dp[i][j] : skip pt
        // i=0:friend turn, i=1:your turn
        // j: いくつのボスを倒したか
        let mut dp = vec![vec![n;n+5];2];
        dp[0][0] = 0;

        for i in 0..n {
            // friend 1
            dp[1][i+1] = min(dp[1][i+1],dp[0][i]+a[i]);
            // friend 2
            dp[1][i+2] = min(dp[1][i+2],dp[0][i]+a[i]+a[i+1]);
            // you 1
            dp[0][i+1] = min(dp[0][i+1],dp[1][i]);
            // you 2
            dp[0][i+2] = min(dp[0][i+2],dp[1][i]);

        }

        writeln!(out, "{}", min(dp[0][n], dp[1][n])).ok();
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
