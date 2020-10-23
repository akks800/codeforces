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
struct Solver {
    s: Vec<usize>,
    v: Vec<usize>,
}

impl Solver {
    fn calc_good(&mut self, l: usize, r: usize, lv: usize) -> usize {
        if r - l == 1 {
            if self.s[l] == lv {
                0
            } else {
                1
            }
        } else {
            let m = l + (r - l) / 2;
            let ans = min(
                self.calc_good(l, m, lv + 1) + self.calc_flat(m, r, lv),
                self.calc_flat(l, m, lv) + self.calc_good(m, r, lv + 1),
            );
            ans
        }
    }
    fn calc_flat(&mut self, l: usize, r: usize, level: usize) -> usize {
        (r - l) - (self.v[r * 26 + level] - self.v[l * 26 + level])
    }

    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        //1385d
        let n = scan.u();
        let s = scan.chars();
        self.s = s.iter().map(|&x| x as usize - 'a' as usize).collect();

        self.v = vec![0usize; (n + 1) * 26];

        let mut ct = vec![0; 26];
        for i in 0..n {
            ct[self.s[i]] += 1;
            self.v[(i + 1) * 26..(i + 2) * 26].copy_from_slice(&ct);
        }

        let x = self.calc_good(0, n, 0);

        writeln!(out, "{}", x).ok();
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
