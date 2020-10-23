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
        //1385e
        let n = scan.u();
        let m = scan.u();

        let mut vu = vec![Vec::new(); n]; // undirected
        let mut vd = vec![Vec::new(); n]; // directed
        let mut refcount = vec![0; n]; // 入次数
        for _ in 0..m {
            let t = scan.u();
            let x = scan.u() - 1;
            let y = scan.u() - 1;

            if t == 0 {
                vu[x].push(y);
            } else {
                vd[x].push(y);
                refcount[y] += 1;
            }
        }

        let mut q = VecDeque::new();
        for i in 0..n {
            q.push_back(i);
        }

        let mut v = vec![0; n];
        let mut p = 1;
        loop {
            // 重い? NlogNぐらい?
            let mut q2 = VecDeque::new();
            let mut modified = false;
            while let Some(x) = q.pop_front() {
                if v[x] != 0 {
                    continue;
                }
                if refcount[x] == 0 {
                    v[x] = p;
                    p += 1;
                    for &y in &vd[x] {
                        refcount[y] -= 1;
                        if refcount[y] == 0 {
                            q.push_front(y);
                        }
                    }
                    modified = true;
                } else {
                    q2.push_back(x);
                }
            }
            if q2.len() == 0 {
                break;
            }
            if modified == false {
                writeln!(out, "NO").ok();
                return;
            }

            q = q2;
        }

        writeln!(out, "YES").ok();
        for x in 0..n {
            for &y in &vu[x] {
                if v[x] < v[y] {
                    writeln!(out, "{} {}", x + 1, y + 1).ok();
                } else {
                    writeln!(out, "{} {}", y + 1, x + 1).ok();
                }
            }
            for &y in &vd[x] {
                assert!(v[x] < v[y]);
                writeln!(out, "{} {}", x + 1, y + 1).ok();
            }
        }
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
