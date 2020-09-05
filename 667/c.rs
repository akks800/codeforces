//use std::cmp::*;
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
        let n: i64 = scan.cin();
        let x: i64 = scan.cin();
        let y: i64 = scan.cin();
        let diff = y - x;
        for i in 1.. {
            if diff % i == 0 && diff / i < n {
                let mut a1 = y - (n - 1) * i;
                while a1 < 1 {
                    a1 += i;
                }

                for j in 0..n {
                    write!(out, "{} ", a1 + j * i).ok();
                }
                writeln!(out, "").ok();
                break;
            }
        }
    }
}