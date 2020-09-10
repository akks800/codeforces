use std::cmp::*;
#[allow(unused_imports)]
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

struct Solver {}
impl Solver {
    fn solve() {
        let mut scan = Scanner::new();
        let out = &mut BufWriter::new(stdout());

        let n: usize = scan.cin();
        let h: Vec<usize> = scan.vec(n);
        const MIN_H: usize = 1;
        const MAX_H: usize = 1000_000_000;
        let mut stk_h = VecDeque::new();
        let mut stk_l = VecDeque::new();
        let badpos = n;
        stk_h.push_back((badpos, MIN_H - 1));
        stk_l.push_back((badpos, MAX_H + 1));
        let mut link = vec![Vec::new(); n + 1];

        for i in 0..n {
            // 1-2-3-2 
            // 1-2-2
            // スタックは単調増加。同じ値を入れない。
            while let Some(&(j, hj)) = stk_h.back() {
                link[j].push(i);
                if h[i] < hj {
                    stk_h.pop_back();
                    continue;
                } else if h[i] == hj {
                    stk_h.pop_back();
                    stk_h.push_back((i, h[i]));
                    break;
                } else {
                    stk_h.push_back((i, h[i]));
                    break;
                }
            }

            while let Some(&(j, hj)) = stk_l.back() {
                link[j].push(i);
                if h[i] > hj {
                    stk_l.pop_back();
                    continue;
                } else if h[i] == hj {
                    stk_l.pop_back();
                    stk_l.push_back((i, h[i]));
                    break;
                } else {
                    stk_l.push_back((i, h[i]));
                    break;
                }
            }
        }

        let mut dp = vec![n;n+1];
        dp[0] = 0;
        for i in 0..n {
            //print!("{}:  ", i);
            for &j in &link[i] {
                dp[j] = min(dp[j], dp[i]+1);
                //print!("{} ", j);
            }
            //println!("");
        }

        writeln!(out, "{}", dp[n-1]).ok();
    }
}

fn main() {
    let th = std::thread::Builder::new().stack_size(64 * 1024 * 1024);
    th.spawn(|| Solver::solve()).unwrap().join().unwrap()
}
