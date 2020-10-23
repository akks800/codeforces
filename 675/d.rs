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
struct Solver {
    p: Vec<IPoint>,
    b: BinaryHeap<Pos>,
    p_step: Vec<i64>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Pos {
    nstep: i64, // -step
    i: usize,
}

#[derive(Clone, Debug, Default)]
struct IPoint {
    i: usize,
    x: i64,
    y: i64,
    neighbor: [(usize, i64); 4],
}

impl Solver {
    fn add(&mut self, i: usize, step: i64) {
        if step < self.p_step[i] {
            self.p_step[i] = step;
            self.b.push(Pos { nstep: -step, i: i });
        }
    }
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        //1422d
        let n = scan.ll();
        let m = scan.u();
        let sx = scan.ll();
        let sy = scan.ll();
        let fx = scan.ll();
        let fy = scan.ll();
        let mut p = Vec::with_capacity(m);
        for i in 0..m {
            let x = scan.ll();
            let y = scan.ll();
            p.push(IPoint {
                i,
                x,
                y,
                neighbor: [(m, 0); 4],
            });
        }

        p.sort_unstable_by_key(|xy| xy.x);
        for i in 0..m {
            if i != 0 {
                p[i].neighbor[0] = (p[i - 1].i, (p[i].x - p[i - 1].x).abs());
            }
            if i != m - 1 {
                p[i].neighbor[1] = (p[i + 1].i, (p[i].x - p[i + 1].x).abs());
            }
        }

        p.sort_unstable_by_key(|xy| xy.y);
        for i in 0..m {
            if i != 0 {
                p[i].neighbor[2] = (p[i - 1].i, (p[i].y - p[i - 1].y).abs());
            }
            if i != m - 1 {
                p[i].neighbor[3] = (p[i + 1].i, (p[i].y - p[i + 1].y).abs());
            }
        }

        p.sort_unstable_by_key(|xy| xy.i);
        self.p = p;

        // dijkstra
        self.p_step = vec![n * 2; m]; // 各instant-movement地点へ到達するまでの最小ステップ
        self.b = BinaryHeap::new();

        // スタートから直接ゴールに移動した場合
        let mut shortest_step = (fx - sx).abs() + (fy - sy).abs();

        // スタートから各地点に移動した場合
        for i in 0..m {
            self.add(i, min((sx - self.p[i].x).abs(), (sy - self.p[i].y).abs()));
        }

        while let Some(pos) = self.b.pop() {
            let i = pos.i;
            let step = -pos.nstep;

            if step > shortest_step {
                break; // もう shortest_step より良い方法は無い
            }
            if self.p_step[i] < step {
                continue; // ここに、もっと短い距離で既に到達している
            }

            // 現地点からゴールに移動した場合
            let expected_step = step + (fx - self.p[i].x).abs() + (fy - self.p[i].y).abs();
            shortest_step = min(shortest_step, expected_step);

            // 現地点から別の地点に移動した場合
            for j in 0..4 {
                let (next_i, dist) = self.p[i].neighbor[j];
                if next_i != m {
                    self.add(next_i, step + dist);
                }
            }
        }

        writeln!(out, "{}", shortest_step).ok();
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
