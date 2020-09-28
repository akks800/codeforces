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
    fn vecu(&mut self, n: usize) -> Vec<usize> {
        self.vec(n)
    }
}

///////

#[allow(unused_macros)]
macro_rules! chmin {
    ($dst:expr, $src:expr) => {{
        let src = $src;
        let dst = &mut $dst;
        let b = *dst > src;
        if b {
            *dst = src;
        }
        b
    }};
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

#[derive(Clone, Debug, Default)]
struct Edge {
    u: usize,
    v: usize,
    weight: usize,
    th0: usize,     // ?-th in out-degree (0-based)
    odegree: usize, // out-degree
}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        // 1394b
        let n = scan.u();
        let m = scan.u();
        let k = scan.u();

        let mut us: Vec<Vec<usize>> = vec![Vec::new(); n];
        let mut vs: Vec<Vec<usize>> = vec![Vec::new(); n];
        // from, to, weight, th, out-degree
        let mut edges: Vec<Edge> = Vec::with_capacity(m);

        for i in 0..m {
            let u = scan.u() - 1;
            let v = scan.u() - 1;
            let weight = scan.u();
            us[u].push(i);
            vs[v].push(i);
            edges.push(Edge {
                u,
                v,
                weight,
                th0: 0,
                odegree: 0,
            });
        }

        for u in 0..n {
            us[u].sort_unstable_by_key(|&edge_id| edges[edge_id].weight);
            let odegree = us[u].len();
            for j in 0..odegree {
                edges[us[u][j]].th0 = j;
                edges[us[u][j]].odegree = odegree;
            }
        }

        let th0_odegree_to_u64 =
            |th0: usize, odegree: usize| 1u64 << (odegree * (odegree - 1) / 2 + th0);

        let mut odegree_th0_to_u64 = vec![vec![0u64; k + 1]; k + 1];
        for odegree in 1..=k {
            for th0 in 0..odegree {
                odegree_th0_to_u64[odegree][th0] = th0_odegree_to_u64(th0, odegree);
            }
        }

        let mut odegree_th0_to_bad_bits = vec![vec![0u64; k + 1]; k + 1];
        for v in 0..n {
            let mut bad_bits = 0u64;
            let mut bad_bad_bits = 0u64;
            for j in 0..vs[v].len() {
                let edge = &edges[vs[v][j]];
                let tmp = th0_odegree_to_u64(edge.th0, edge.odegree);
                if bad_bits & tmp == 0 {
                    bad_bits |= tmp;
                } else {
                    // この (th0, odegree)の組み合わせは、自分自身と衝突している
                    bad_bad_bits |= tmp;
                }
            }

            for j in 0..vs[v].len() {
                let edge = &edges[vs[v][j]];
                let tmp = th0_odegree_to_u64(edge.th0, edge.odegree);
                if bad_bad_bits & tmp == 0 {
                    odegree_th0_to_bad_bits[edge.odegree][edge.th0] |= bad_bits & !tmp;
                } else {
                    odegree_th0_to_bad_bits[edge.odegree][edge.th0] |= bad_bits;
                }
            }
        }

        let mut c = vec![0; k + 1];
        fn dfs(
            odegree: usize,
            k: usize,
            odegree_th_to_u64: &Vec<Vec<u64>>,
            odegree_th0_to_bad_bits: &Vec<Vec<u64>>,
            current_bits: u64,
            bad_bits: u64,
            c: &mut Vec<usize>,
        ) -> u64 {
            if odegree == k + 1 {
                //println!( "{:?}", &c[1..] );
                return 1;
            } else {
                let mut ans = 0u64;
                for th in 0..odegree {
                    c[odegree] = th + 1;
                    let new_current_bits = current_bits | odegree_th_to_u64[odegree][th];
                    let new_bad_bits = bad_bits | odegree_th0_to_bad_bits[odegree][th];
                    if new_current_bits & new_bad_bits != 0 {
                        continue;
                    }
                    ans += dfs(
                        odegree + 1,
                        k,
                        odegree_th_to_u64,
                        odegree_th0_to_bad_bits,
                        new_current_bits,
                        new_bad_bits,
                        c,
                    )
                }
                return ans;
            }
        }

        let ans = dfs(
            1,
            k,
            &odegree_th0_to_u64,
            &odegree_th0_to_bad_bits,
            0,
            0,
            &mut c,
        );

        writeln!(out, "{} ", ans).ok();
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