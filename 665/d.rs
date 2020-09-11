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

const MOD: u64 = 1000_000_007;
#[derive(Clone, Debug, Default)]
struct Solver {
    e: Vec<Vec<usize>>,
    n: usize,
}

impl Solver {
    fn dfs(&self, node: usize, parent: usize, v: &mut Vec<u64>) -> u64 {
        let mut ct = 1;
        for &next_node in &self.e[node] {
            if next_node != parent {
                let x = self.dfs(next_node, node, v);
                v.push(x * (self.n as u64 - x));
                ct += x;
            }
        }
        ct
    }
    fn solve(&mut self, scan: &mut Scanner) -> u64 {
        let n: usize = scan.cin();
        self.n = n;

        self.e = vec![Vec::new(); n];
        for _ in 0..n - 1 {
            let u: usize = scan.cin();
            let v: usize = scan.cin();
            self.e[u - 1].push(v - 1);
            self.e[v - 1].push(u - 1);
        }
        // edge_weight : edgeが何回カウントされるか
        let mut edge_weight = Vec::with_capacity(n - 1);
        self.dfs(0, n, &mut edge_weight);
        edge_weight.sort_unstable();
        for i in 0..n - 1 {
            edge_weight[i] %= MOD;
        }

        let mut edge_val = vec![1; n - 1];
        {
            let np: usize = scan.cin();
            let mut p: Vec<u64> = scan.vec(np);
            p.sort_unstable();

            let mut idx = 0;
            if np < n - 1 {
                idx = n - 1 - np;
            }

            for &pval in &p {
                edge_val[idx] *= pval;
                edge_val[idx] %= MOD;
                idx = min(idx + 1, n - 2);
            }
        }

        let mut ans = 0;
        for i in 0..n - 1 {
            //println!("{} {}", edge_val[i], edge_weight[i]);
            ans = (ans + edge_val[i] * edge_weight[i]) % MOD;
        }

        ans
    }
}

fn solve() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    //let testcase = 1;
    let testcase: usize = scan.cin();
    for _ in 0..testcase {
        let mut s: Solver = Default::default();
        let ans = s.solve(&mut scan);
        writeln!(out, "{}", ans).ok();
    }
}

fn main() {
    let th = std::thread::Builder::new().stack_size(64 * 1024 * 1024);
    th.spawn(|| solve()).unwrap().join().unwrap()
}
