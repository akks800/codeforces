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
}

///////

#[derive(Clone, Debug, Default)]
struct Solver {}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        let n: usize = scan.cin();
        let mut a: Vec<i32> = scan.vec(n);
        let l: Vec<i32> = scan.vec(n);

        let sum: i32 = a.iter().sum();
        if sum >= 0 {
            // 正の数を先に、負の数を後に
            // ロックがあっても方針は変わらない
            let mut v = Vec::with_capacity(n);
            for i in 0..n {
                if l[i] == 0 {
                    v.push(a[i]);
                }
            }
            v.sort_unstable();

            for i in 0..n {
                if l[i] == 0 {
                    a[i] = v.pop().unwrap();
                }
            }
        } else {
            // pn < 0 の場合はどうすることもできない
        }

        for i in 0..n {
            write!(out, "{} ", a[i]).ok();
        }

        writeln!(out, "").ok();
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
