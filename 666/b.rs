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

fn calc_distance(c: i64, a: &Vec<i64>) -> i64 {
    let mut x = 1i64;
    let n = a.len();
    let mut diff = 0;
    let max = std::i64::MAX / c;
    for i in 0..n {
        diff += (x - a[i]).abs();
        if i != n - 1 {
            if max < x {
                return std::i64::MAX;
            }
            x *= c;
        }
    }

    //println!("{} {}", c, diff);
    diff
}

fn main() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let n: usize = scan.cin();
    let mut a: Vec<i64> = scan.vec(n);

    a.sort();

    let mut l = 1;
    let mut h = 100000;
    let mut ans = std::i64::MAX;
    loop {
        let m1 = l + (h - l) / 2;
        let m0 = l + (m1 - l) / 2;
        let m2 = h - (h - m1) / 2;

        let s0 = calc_distance(m0, &a);
        let s1 = calc_distance(m1, &a);
        let s2 = calc_distance(m2, &a);

        ans = min(ans, s0);
        ans = min(ans, s1);
        ans = min(ans, s2);

        if h - l <= 4 {
            break;
        }

        if s0 > s1 {
            l = m0;
            if s1 > s2 {
                l = m1;
            }
        }

        if s2 > s1 {
            h = m2;
            if s1 > s0 {
                h = m1;
            }
        }

        if s1 == std::i64::MAX {
            h = m1;
        }
        if s0 == std::i64::MAX {
            h = m0;
        }
        //println!("{} {}", l, h);
    }

    writeln!(out, "{}", ans).ok();
}
