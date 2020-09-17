use std::cmp::*;
#[allow(unused_imports)]
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
}

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
    fn new(n: usize, unit: T, comb: fn(T, T) -> T) -> SegmentTree<T> {
        let mut x = 1;
        while x < n {
            x *= 2;
        }

        SegmentTree {
            v: vec![unit; x * 2],
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

    fn range(&self, begin: usize, end: usize) -> T {
        self.range_sub(begin, end, 1, 0, self.offset)
    }

    // v[node] は 区間 l .. r-1 の計算結果を持っている
    fn range_sub(&self, begin: usize, end: usize, node: usize, l: usize, r: usize) -> T {
        if end <= l || r <= begin {
            return self.unit.clone();
        } else if begin <= l && r <= end {
            return self.v[node].clone();
        } else {
            let lval = self.range_sub(begin, end, node * 2, l, (l + r) / 2);
            let rval = self.range_sub(begin, end, node * 2 + 1, (l + r) / 2, r);
            return (self.comb)(lval, rval);
        }
    }
}

#[derive(Clone, Copy)]
struct PosOccurance {
    pos: usize,
    occ: usize,
}

fn main() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let n = scan.cin();
    let a: Vec<usize> = scan.vec(n);

    type Binop = fn(PosOccurance, PosOccurance) -> PosOccurance;
    let bo: Binop = |l, r| if l.occ <= r.occ { l } else { r };
    let po0 = PosOccurance {
        pos: n,
        occ: 1000_000_001,
    };
    let mut s = SegmentTree::new(n, po0, bo);

    for pos in 0..n {
        s.update(pos, PosOccurance { pos, occ: a[pos] });
    }

    fn dfs(
        s: &SegmentTree<PosOccurance>,
        a: &Vec<usize>,
        mut l: usize,
        r: usize,
        sub: usize,
    ) -> usize {
        while l < r && a[l] == sub {
            l += 1;
        }
        if l == r {
            0
        } else if l + 1 == r {
            1
        } else {
            let po = s.range(l, r);
            // operation 1
            let op1 = po.occ - sub;
            let ans1 = op1 + dfs(s, a, l, po.pos, po.occ) + dfs(s, a, po.pos + 1, r, po.occ);

            // operation 2
            let ans2 = r - l;

            //println!("{} {} {} : {} ", l, r, sub, min(ans1, ans2));
            min(ans1, ans2)
        }
    }

    let ans = dfs(&s, &a, 0, n, 0);

    writeln!(out, "{}", ans).ok();
}
