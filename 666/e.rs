#[allow(unused_imports)]
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

fn main() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let n: usize = scan.cin();
    let r1: i64 = scan.cin();
    let r2: i64 = scan.cin();
    let r3: i64 = scan.cin();
    let d: i64 = scan.cin();
    let a: Vec<i64> = scan.vec(n);

    let mut dp0 = vec![0i128; n + 1]; // time to clear [1,i]
    let mut dp2 = vec![0i128; n + 1]; // time to clear [1,i] // 一度この階に来たことがある
    dp0[0] = -d as i128;
    dp2[0] = d as i128 * 2;

    for i in 1..=n {
        let rr2 = min(r2, (a[i - 1] + 1) * r1);
        let normal_cost = a[i - 1] * r1 + r3;

        let cost00 = d + min(normal_cost, rr2 + d + d + r1);
        let cost02 = d + rr2 + d + d + r1;
        let cost20 = 
            if i == n {
                min(normal_cost, rr2 + d + r1)
            } else {
                min(d + normal_cost, rr2 + d + r1)
            };

        dp0[i] = min(dp0[i - 1] + cost00 as i128, dp2[i - 1] + cost20 as i128);
        dp2[i] = dp0[i - 1] + cost02 as i128;
        //println!(
        //    "0:{} 2:{} / 00:{} 02:{} 20:{}",
        //    dp0[i], dp2[i], cost00, cost02, cost20
        //);
    }
    writeln!(out, "{} ", dp0[n]).ok();
}
