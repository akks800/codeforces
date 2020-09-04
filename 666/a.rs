#[allow(unused_imports)]
use std::collections::*;
use std::str::*;
use std::io::*;

// scanner from https://codeforces.com/contest/1396/submission/91365784
struct Scanner {
    stdin: Stdin,
    buffer: VecDeque<String>,
}
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

    let t: usize = scan.cin();
    for _tt in 0..t {
        let n: usize = scan.cin();
        let mut h = vec![0; 26];
        for _i in 0..n {
            let v = scan.chars();
            for ch in v {
                h[ch as usize - 'a' as usize] += 1;
            }
        }
        let b = h.iter().all(|val| val % n == 0);
        writeln!(out, "{}", if b { "YES" } else { "NO" }).ok();
    }
}
