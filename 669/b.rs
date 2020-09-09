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
        let mut hist = vec![0; 1005];
        for i in 0..n {
            hist[a[i]] += 1;
        }

        // c1 = maxval, c2 = maxgcd
        let mut v = Vec::new();
        let mut c = *a.iter().max().unwrap();
        loop {
            for i in (c..=1000).step_by(c) {
                while hist[i] != 0 {
                    v.push(i);
                    hist[i] -= 1;
                }
            }
            if c == 1 {
                break;
            }

            // find next c = max gcd(g,?)
            let mut next_c = 1;
            'outer: for i in 2..=c {
                if c % i == 0 {
                    let gcdval = c / i;
                    for j in (gcdval..=1000).step_by(gcdval) {
                        if hist[j] != 0 {
                            next_c = gcdval;
                            break 'outer;
                        }
                    }
                }
            }
            c = next_c;
        }

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