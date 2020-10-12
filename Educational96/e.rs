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

// http://www.prefield.com/algorithm/sort/mergecount.html
fn f(a: &mut Vec<usize>) -> u64 {
    let mut ct = 0u64;
    let n = a.len();
    if n > 1 {
        let mut b = Vec::with_capacity(n / 2 + 1);
        let mut c = Vec::with_capacity(n / 2 + 1);
        for i in 0..n {
            if i < n / 2 {
                b.push(a[i]);
            } else {
                c.push(a[i]);
            }
        }
        ct += f(&mut b);
        ct += f(&mut c);

        let mut j = 0;
        let mut k = 0;
        for i in 0..n {
            if k == c.len() {
                a[i] = b[j];
                j += 1;
            } else if j == b.len() {
                a[i] = c[k];
                k += 1;
            } else if b[j] <= c[k] {
                a[i] = b[j];
                j += 1;
            } else {
                a[i] = c[k];
                k += 1;
                ct += (n / 2 - j) as u64;
            }
        }
    }

    ct
}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        //1430e

        let n = scan.u();
        let s = scan.chars();
        let mut rs = s.clone();
        rs.reverse();

        let mut apos = vec![Vec::new(); 26];
        for i in 0..n {
            let ch = s[i] as usize - 'a' as usize;
            apos[ch].push(i);
        }

        // 並び替えた後の文字列の末尾から順に
        let mut pos_to_pos = vec![0; n];
        for new_pos in (0..n).rev() {
            let ch = rs[new_pos] as usize - 'a' as usize;
            let old_pos = apos[ch].pop().unwrap();
            pos_to_pos[old_pos] = new_pos;
        }

        // バブルソートの交換回数
        writeln!(out, "{}", f(&mut pos_to_pos)).ok();
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
