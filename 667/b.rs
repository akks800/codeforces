//use std::cmp::*;
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
        let a: i64 = scan.cin();
        let b: i64 = scan.cin();
        let x: i64 = scan.cin();
        let y: i64 = scan.cin();
        let n: i64 = scan.cin();

        fn f(mut a:i64, mut b:i64, x:i64, y:i64, mut n:i64) ->i64{
            let d = min(n, a-x);
            a -= d;
            n -= d;

            if n > 0 {
                let d = min(n, b-y);
                b -= d;
            }
            a*b
        }

        let p = f(a, b, x, y, n);
        let q = f(b, a, y, x, n);

        writeln!(out, "{}", min(p,q)).ok();
    }
}