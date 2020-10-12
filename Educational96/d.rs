//use std::cmp::*;
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
    fn u(&mut self) -> usize {
        self.cin()
    }
    fn ll(&mut self) -> i64 {
        self.cin()
    }
    fn ull(&mut self) -> u64 {
        self.cin()
    }
    fn vecu(&mut self, n: usize) -> Vec<usize> {
        self.vec(n)
    }
    fn vecll(&mut self, n: usize) -> Vec<i64> {
        self.vec(n)
    }
}

#[derive(Clone, Debug, Default)]
struct Solver {}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        //1430d

        let n = scan.u();
        let s = scan.chars();

        // s を、同じ文字が何個連続するか、に変換する
        let mut v = VecDeque::with_capacity(n);
        v.push_back(1);
        for i in 1..n {
            if s[i] == s[i - 1] {
                let len = v.len();
                v[len - 1] += 1;
            } else {
                v.push_back(1);
            }
        }

        let mut op = 0;
        let mut pos_top = 0;
        let mut pos_del = 0;
        while pos_top < v.len() {
            op += 1;

            if pos_del < pos_top {
                pos_del = pos_top;
            }

            // 削除してもいい場所(同じ文字が連続する場所)を探す
            while pos_del < v.len() && v[pos_del] < 2 {
                pos_del += 1;
            }

            if pos_del != v.len() {
                v[pos_del] -= 1;
                pos_top += 1;
            } else {
                // 削除できる場所がない。1010101010... になっている。
                pos_top += 2;
            }
        }

        writeln!(out, "{}", op).ok();
    }
}

fn solve() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    //let testcase = 1;
    let testcase: usize = scan.cin();
    for _ in 0..testcase {
        let mut s: Solver = Default::default();
        s.solve(&mut scan, out);
    }
}

fn main() {
    let th = std::thread::Builder::new().stack_size(64 * 1024 * 1024);
    th.spawn(|| solve()).unwrap().join().unwrap()
}
