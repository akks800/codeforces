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

fn is_ok(v: &[i64], l: i64, k: i64) -> bool {
    let mut start_time = 0;
    let mut end_time = k * 4;

    // start_time..=end_time に v[0] を出発すれば渡れる
    for i in 0..v.len() {
        // start_time = k*2-p, v[0]+p = l
        start_time = max(start_time, k * 2 + v[i] - l - i as i64);
        // end_time = k*2+p, v[0]+p = l
        end_time = min(end_time, k * 2 - v[i] + l - i as i64);
    }
    start_time <= end_time
}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        //1384b2
        let n = scan.u();
        let k = scan.ll();
        let l = scan.ll();
        let v = scan.vecll(n);

        let mut pos = 0;
        /*
          安全な場所から 次の安全な場所に移動できるか？
          移動は好きなタイミングで開始できる
        */
        while pos < n {
            // 安全な場所(常に溺れない場所)をスキップ
            while pos < n && v[pos] + k <= l {
                pos += 1;
            }
            if pos == n {
                break;
            }
            let pos0 = pos;

            // 安全でない場所がどこまで続くか
            while pos < n && v[pos] + k > l {
                if v[pos] > l {
                    writeln!(out, "No").ok();
                    return;
                }
                pos += 1;
            }
            let pos1 = pos;

            // pos0..pos1 が安全でない領域
            if !is_ok(&v[pos0..pos1], l, k) {
                writeln!(out, "No").ok();
                return;
            }
        }

        writeln!(out, "Yes").ok();
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
