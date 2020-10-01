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
    fn uu(&mut self) -> u64 {
        self.cin()
    }
    fn vecu(&mut self, n: usize) -> Vec<usize> {
        self.vec(n)
    }
    fn vecll(&mut self, n: usize) -> Vec<i64> {
        self.vec(n)
    }
}

#[allow(unused_macros)]
macro_rules! chmax {
    ($dst:expr, $src:expr) => {{
        let src = $src;
        let dst = &mut $dst;
        let b = *dst < src;
        if b {
            *dst = src;
        }
        b
    }};
}

#[derive(Clone, Debug, Default)]
struct Solver {}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        // 1389b
        let n = scan.u();
        let k = scan.u();
        let z = scan.u();
        let a = scan.vecll(n);

        // dpl[i][j], dpr[i][j]: i-step, move left j times // position=i-j*2
        let mut dpl = vec![vec![0; z + 1]; k + 1];
        let mut dpr = vec![vec![0; z + 1]; k + 1];
        dpl[0][0] = a[0];

        for i in 0..k {
            let mut vl = Vec::new();
            let mut vr = Vec::new();
            for j in 0..=z {

                // 右移動  i,j -> i+1,j
                // if 0 <= i-j*2 && i+1-j*2 < n
                if j * 2 <= i && i + 1 < n + j * 2 {
                    chmax!(dpl[i + 1][j], dpl[i][j] + a[i + 1 - j * 2]);
                    chmax!(dpl[i + 1][j], dpr[i][j] + a[i + 1 - j * 2]);
                    vr.push((i,j));
                }
                // 左移動 i,j -> i+1,j+1
                // if j != z && 0 <= i+1-(j+1)*2 && i-j*2 < n
                if j != z && (j + 1) * 2 <= i + 1 && i < n + j * 2 {
                    chmax!(dpr[i + 1][j + 1], dpl[i][j] + a[i + 1 - (j + 1) * 2]);
                    vl.push((i,j));
                }
            }
        }
        let ans = max(dpl[k].iter().max().unwrap(), dpr[k].iter().max().unwrap());
        writeln!(out, "{}", ans).ok();
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
