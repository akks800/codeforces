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

//
// Segment Tree
//

#[derive(Clone)]
struct SegmentTree<T> {
    v: Vec<T>,
    offset: usize,
    comb: fn(T, T) -> T,
    unit: T,
}

#[allow(dead_code)]
impl<T: Copy> SegmentTree<T> {
    fn new(n: usize, unit: T, comb: fn(T, T) -> T, vv: Option<&Vec<T>>) -> SegmentTree<T> {
        let mut x = 1;
        while x < n {
            x *= 2;
        }

        let mut v = vec![unit; x * 2];
        if let Some(vv) = vv {
            for i in 0..n {
                v[x + i] = (*vv)[i];
            }
            for i in (0..x).rev() {
                v[i] = comb(v[i * 2], v[i * 2 + 1]);
            }
        }

        SegmentTree {
            v: v,
            offset: x,
            comb,
            unit: unit,
        }
    }

    fn update(&mut self, pos: usize, val: T) {
        let mut p = self.offset + pos;
        self.v[p] = val;
        while p != 1 {
            p /= 2;
            self.v[p] = (self.comb)(self.v[p * 2], self.v[p * 2 + 1]);
        }
    }

    fn get(&self, pos: usize) -> T {
        self.v[self.offset + pos]
    }

    fn query_range_all(&self) -> T {
        self.query_range(0, self.offset)
    }

    fn query_range(&self, begin: usize, end: usize) -> T {
        self.query_range_sub(begin, end, 1, 0, self.offset)
    }

    // v[node] は 区間 l .. r-1 の計算結果を持っている
    fn query_range_sub(&self, begin: usize, end: usize, node: usize, l: usize, r: usize) -> T {
        if end <= l || r <= begin {
            return self.unit.clone();
        } else if begin <= l && r <= end {
            return self.v[node].clone();
        } else {
            let lval = self.query_range_sub(begin, end, node * 2, l, (l + r) / 2);
            let rval = self.query_range_sub(begin, end, node * 2 + 1, (l + r) / 2, r);
            return (self.comb)(lval, rval);
        }
    }
}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        //1430e

        let n = scan.u();
        let s = scan.chars();

        let s: Vec<usize> = s.iter().map(|&x| x as usize - 'a' as usize).collect();
        let mut apos = vec![VecDeque::new(); 26];
        for i in 0..n {
            apos[s[i]].push_back(i);
        }
        // バブルソートの交換回数
        let v = vec![1i64; n];
        let mut sum = 0i64;
        let mut seg = SegmentTree::new(n, 0i64, |l, r| l + r, Some(&v));
        for i in 0..n {
            let ch = s[n - 1 - i];
            let pos = apos[ch].pop_front().unwrap();
            seg.update(pos, 0);
            sum += seg.query_range(0, pos);
        }
        writeln!(out, "{}", sum).ok();
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
