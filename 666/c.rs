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
    let a: Vec<i64> = scan.vec(n);

    if n == 1 {
        writeln!(out, "{} {}", 1, 1).ok();
        writeln!(out, "{}", -a[0]).ok();
        writeln!(out, "{} {}", 1, 1).ok();
        writeln!(out, "{}", 0).ok();
        writeln!(out, "{} {}", 1, 1).ok();
        writeln!(out, "{}", 0).ok();
    } else {
        writeln!(out, "{} {}", 1, n-1).ok();
        write!(out, "{} ", a[0]*(n-1) as i64).ok();
        for _i in 1..n-1 {
            write!(out, "{} ", 0).ok();
        }
        writeln!(out, "").ok();

        writeln!(out, "{} {}", 2, n).ok();
        for i in 1..n {
            write!(out, "{} ", a[i]*(n-1) as i64).ok();
        }
        writeln!(out, "").ok();
        
        writeln!(out, "{} {}", 1, n).ok();
        for i in 0..n {
            write!(out, "{} ", -a[i]*n as i64).ok();
        }
        writeln!(out, "").ok();
    }
}
