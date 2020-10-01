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
}

#[derive(Clone, Debug, Default)]
struct Solver {}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        // 1389d
        let n = scan.uu();
        let mut k = scan.uu();

        let l1 = scan.uu();
        let r1 = scan.uu();
        let l2 = scan.uu();
        let r2 = scan.uu();

        let lmax = max(l1, l2);
        let lmin = min(l1, l2);
        let rmax = max(r1, r2);
        let rmin = min(r1, r2);

        // 接触するまでの距離
        let distance = if r2 <= l1 {
            l1 - r2
        } else if r1 <= l2 {
            l2 - r1
        } else {
            0
        };

        // 既に重なっている部分
        let intersected = {
            if lmax < rmin {
                rmin - lmax
            } else {
                0
            }
        };

        // 接触後、1ステップでIを+1できる回数
        let space = (rmax - lmin) - intersected;

        let ans;
        if k <= intersected * n {
            ans = 0;
        } else {
            k -= intersected * n;
            let mut v = Vec::new();

            let completed_pair = if space == 0 { n } else { min(n, k / space) };
            // 1つ以上のcompleted_pair と、既存のペアを伸ばす場合
            if completed_pair != 0 {
                v.push(completed_pair * (distance + space) + (k - (completed_pair * space)) * 2);
            }

            // n個未満のcompleted_pair と、追加で1つ使う場合
            if completed_pair != n {
                v.push(
                    completed_pair * (distance + space) + distance + (k - (completed_pair * space)),
                );
            }
            ans = *v.iter().min().unwrap();
        }

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
