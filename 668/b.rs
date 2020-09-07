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

fn main() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());
    let testcase: usize = scan.cin();
    for _ in 0..testcase {
        let n: usize = scan.cin();
        let mut a: Vec<i64> = scan.vec(n);

        let mut lp = 0; // a[lp] positive
        let mut rp = n; // a[rp-1] positive
        let mut ln = 0; // a[ln] negative
        let mut ans = 0;
        loop {
            while ln < n && a[ln] >= 0 {
                ln += 1;
            }
            if ln == n {
                break;
            }

            while lp < ln && a[lp] <= 0 {
                lp += 1;
            }
            while rp > 0 && a[rp - 1] <= 0 {
                rp -= 1;
            }

            // a[ln] < 0, lp<=ln<n
            if a[lp] > 0 {
                let op = min(a[lp], -a[ln]);
                a[lp] -= op;
                a[ln] += op;
            } else {
                let op = min(a[rp - 1], -a[ln]);
                a[rp - 1] -= op;
                a[ln] += op;
                ans += op;
            }
        }
        writeln!(out, "{}", ans).ok();
    }
}
