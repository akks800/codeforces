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

#[derive(Clone, Debug)]
struct Solver {}

fn solve() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let testcase: usize = scan.cin();
    for _ in 0..testcase {
        let n: usize = scan.cin();
        let a: Vec<usize> = scan.vec(n);

        let mut v = Vec::new();
        for i in (0..n).step_by(2) {
            if a[i] == a[i + 1] {
                v.push(a[i]);
                v.push(a[i + 1]);
            } else {
                v.push(0);
            }
        }
        writeln!(out, "{}", v.len()).ok();
        for i in v {
            write!(out, "{} ", i).ok();
        }
        writeln!(out, "").ok();
    }
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024) // 32 MBのスタックサイズ
        .spawn(|| {
            solve(); // ここで深い再帰を実行
        })
        .unwrap()
        .join()
        .unwrap();
}