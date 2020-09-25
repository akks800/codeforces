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
    fn vecu(&mut self, n: usize) -> Vec<usize> {
        self.vec(n)
    }
}

///////

#[allow(unused_macros)]
macro_rules! chmin {
    ($dst:expr, $src:expr) => {{
        let src = $src;
        let dst = &mut $dst;
        let b = *dst > src;
        if b {
            *dst = src;
        }
        b
    }};
}

#[derive(Clone, Debug, Default)]
struct Solver {}

fn f(a: &mut Vec<usize>, len: usize) -> usize {
    a.push(a[0]);
    a.push(a[1]);

    let mut dp = vec![len; 4];
    dp[a[0] * 2 + a[1]] = 0;

    for i in 2..len + 2 {
        let mut dpnew = vec![len; 4];
        for j in 0..4 {
            let x = j * 2 + a[i];
            if x != 0 && x != 7 {
                chmin!(dpnew[x & 3], dp[j]);
            }
            if i < len {
                let y = j * 2 + (a[i] ^ 1);
                if y != 0 && y != 7 {
                    chmin!(dpnew[y & 3], dp[j] + 1);
                }
            }
        }
        dp = dpnew;
    }

    a.pop();
    a.pop();
    dp.into_iter().min().unwrap()
}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        let len: usize = scan.cin();
        let a = scan.chars();
        let mut b: Vec<usize> = a.iter().map(|&c| if c == 'L' { 1 } else { 0 }).collect();

        let ans1 = f(&mut b, len);
        b[0] ^= 1;
        let ans2 = f(&mut b, len) + 1;
        b[1] ^= 1;
        let ans3 = f(&mut b, len) + 2;
        b[0] ^= 1;
        let ans4 = f(&mut b, len) + 1;
        //println!("{} {} {} {}", ans1, ans2, ans3, ans4);

        writeln!(out, "{}", [ans1, ans2, ans3, ans4].iter().min().unwrap()).ok();
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
