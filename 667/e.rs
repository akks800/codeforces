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
        let n: usize = scan.cin();
        let k: usize = scan.cin();
        let mut x: Vec<usize> = scan.vec(n);
        let _y: Vec<usize> = scan.vec(n);
        x.sort_unstable();
        let mut v = Vec::with_capacity(n);
        for i in 0..n {
            v.push((x[i], 0, 0));
        }

        // v[l].1 : v[l].0 に左端が接するように置くと、次に救えないポイントはどこか?
        let mut r = 0;
        for l in 0..n {
            while r < n && v[r].0 <= v[l].0 + k {
                r += 1;
            }
            v[l].1 = r;
        }
        // v[i].2 : v[i]あるいはそれ以降で、最大いくつのポイントを救えるか?
        let mut max_save = 0;
        for i in (0..n).rev() {
            max_save = max(max_save, v[i].1 - i);
            v[i].2 = max_save;
        }

        let mut ans = 0;
        for l in 0..n {
            let r = v[l].1;
            let save1 = r - l;
            let save2 = if r == n { 0 } else { v[r].2 };
            ans = max(ans, save1 + save2);
        }

        writeln!(out, "{} ", ans).ok();
    }
}