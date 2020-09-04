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

    let test: usize = scan.cin();
    for _ in 0..test {
        let n: usize = scan.cin();
        let mut a: Vec<i64> = scan.vec(n);
        a.sort();
        let sum: i64 = a.iter().sum();
        let largest = a[a.len() - 1];
        let other = sum - largest;
        if largest > other {
            // largest を取り続けて勝ち
            writeln!(out, "{} ", "T").ok();
        } else {
            writeln!(out, "{} ", if sum % 2 == 0 { "HL" } else { "T" }).ok();
        }
    }
}
