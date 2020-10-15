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
struct Solver {}

// a & fixed_bits が同じ値で連続する各部分の、a&bit と !a&bit の inversion の合計を求める
fn count_inversion(a: &Vec<usize>, bit: usize, fixed_bits: usize) -> (u64, u64) {
    let mut l = 0;
    let mut inv0 = 0; // a&bit のinversion
    let mut inv1 = 0; // !a&bit のinversion
    while l < a.len() {
        let mut r = l;
        while r < a.len() && a[l] & fixed_bits == a[r] & fixed_bits {
            r += 1;
        }
        let mut ct0 = 0;
        let mut ct1 = 0;
        for i in l..r {
            if a[i] & bit == 0 {
                ct0 += 1;
                inv0 += ct1;
            } else {
                ct1 += 1;
                inv1 += ct0;
            }
        }
        l = r;
    }
    (inv0, inv1)
}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        //1416c
        let n = scan.u();
        let mut a = scan.vecu(n);
        let mut x = 0;
        let mut inv = 0;
        let mut fixed_bits = 0;
        for i in (0..32).rev() {
            let bit = 1 << i;
            let (ct0, ct1) = count_inversion(&a, bit, fixed_bits);

            if ct1 < ct0 {
                x |= bit;
            }
            inv += min(ct0, ct1);

            fixed_bits |= bit;
            a.sort_by_key(|x| x & fixed_bits); // stable sort
        }

        writeln!(out, "{} {}", inv, x).ok();
    }
}

fn solve() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let testcase = 1;
    //let testcase: usize = scan.cin();
    for _ in 0..testcase {
        let mut s: Solver = Default::default();
        s.solve(&mut scan, out);
    }
}

fn main() {
    let th = std::thread::Builder::new().stack_size(64 * 1024 * 1024);
    th.spawn(|| solve()).unwrap().join().unwrap()
}
