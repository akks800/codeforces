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
        //1427d
        let n = scan.u();
        let a = scan.vecu(n);
        let mut op = Vec::new();
        let mut a: Vec<usize> = a.iter().map(|&x| x - 1).collect();

        for i in (0..n).rev() {
            for pos in 0..n {
                if a[pos] == i {
                    //println!("{:?}", a);
                    //println!("i={} pos={}", i, pos);
                    let mut v = Vec::new();
                    let mut f = |l, r| {
                        if l != r {
                            v.push((l, r))
                        }
                    };
                    if i % 2 == 1 {
                        // 末尾 i+1..n がソート済み → 先頭へ
                        // 0..pos, pos..i+1, i+1..n
                        f(0, pos);
                        f(pos, i + 1);
                        for j in i + 1..n {
                            f(j, j + 1);
                        }
                    } else {
                        // 先頭0..n-1-iがソート済み → 末尾へ
                        // 0..n-1-i, n-1-i..pos+1, pos+1..n
                        for j in 0..n - 1 - i {
                            f(j, j + 1);
                        }
                        f(n - 1 - i, pos + 1);
                        f(pos + 1, n);
                    }

                    //println!("{:?} {:?}", a, v);
                    if v.len() >= 2 {
                        let mut tmp = Vec::with_capacity(n);
                        for &(l, r) in v.iter().rev() {
                            for j in l..r {
                                tmp.push(a[j]);
                            }
                        }
                        a = tmp;
                        op.push(v);
                    }
                    break;
                }
            }
        }
        writeln!(out, "{}", op.len()).ok();
        for v in op {
            write!(out, "{} ", v.len()).ok();
            for (l, r) in v {
                write!(out, "{} ", r - l).ok();
            }
            writeln!(out, "").ok();
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