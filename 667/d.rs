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

    fn f(mut x: u64) -> u64 {
        let mut sum = 0;
        while x != 0 {
            sum += x % 10;
            x /= 10;
        }
        sum
    }

    for _ in 0..testcase {
        let mut n: u64 = scan.cin();
        let s: u64 = scan.cin();

        let mut x = 1;
        let mut ans = 0;
        while f(n) > s {
            let m = (x * 10) - (n % (x * 10));
            n += m;
            x *= 10;
            //println!("{}", n);
            ans += m;
        }
        writeln!(out, "{} ", ans).ok();
    }
}