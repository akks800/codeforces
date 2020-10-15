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

/*
01xxx0111      xi=x<<i
        1yyy1  a
01xxx1000yyy1  a+xi
01xxx0110yyy1  a^xi
1xxx0111yyy10  a+xi + a^xi = g
        yyy10  (a+xi + a^xi) ^ (xi<<1) = h

*/

//
fn f(a: u64, x: u64, v: &mut Vec<(char, u64, u64)>) -> u64 {
    for i in (1..32).rev() {
        let bit = 1 << i;
        if a & bit != 0 {
            let xi = x << i;
            let d = a + xi;
            let e = a ^ xi;
            let g = d + e;
            let mut h = g ^ (xi << 1);
            v.push(('+', a, xi));
            v.push(('^', a, xi));
            v.push(('+', d, e));
            v.push(('^', g, (xi << 1)));

            loop {
                if (a ^ h) < a && (a ^ h) < h {
                    // aとhの最上位ビットが同じ位置にある
                    break;
                }
                v.push(('+', h, h));
                h = h + h;
            }
            v.push(('^', a, h));
            return a ^ h;
        }
    }

    panic!();
}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        //1427e
        let x = scan.ull();
        let mut bs = BTreeSet::new();

        let mut v = Vec::new();
        bs.insert(x);
        for i in 0..32 {
            v.push(('+', x << i, x << i));
            bs.insert(x << (i + 1));
        }

        let mut y = x;
        while y != 1 {
            y = f(y, x, &mut v);
        }

        writeln!(out, "{}", v.len()).ok();
        for (ch, a, b) in v {
            assert!(bs.contains(&a));
            assert!(bs.contains(&b));
            if ch == '+' {
                bs.insert(a + b);
            } else {
                bs.insert(a ^ b);
            }
            writeln!(out, "{} {} {}", a, ch, b).ok();
        }
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
