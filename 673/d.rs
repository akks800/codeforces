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

//
// DSU
//

#[derive(Debug, Clone)]
struct DSU {
    v: Vec<usize>,
    n: usize,
}

impl DSU {
    fn new(size: usize) -> DSU {
        let mut v = vec![0; size];
        for i in 0..size {
            v[i] = i;
        }
        DSU { v: v, n: size }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.v[x] == x {
            x
        } else {
            self.v[x] = self.find(self.v[x]);
            self.v[x]
        }
    }

    fn is_same(&mut self, x: usize, y: usize) -> bool {
        self.find(x) == self.find(y)
    }

    fn unify(&mut self, x: usize, y: usize) {
        let xx = self.find(x);
        let yy = self.find(y);
        if xx != yy {
            self.v[yy] = xx;
        }
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
        //1416d

        let n = scan.u();
        let m = scan.u();
        let q = scan.u();
        let p = scan.vecu(n);

        let mut edges = Vec::with_capacity(m);
        for _ in 0..m {
            let x = scan.u() - 1;
            let y = scan.u() - 1;
            edges.push((x, y));
        }

        let mut vq = Vec::with_capacity(q);
        let mut vq1 = Vec::with_capacity(q); // vertex : そのグループ内の最大のpを0にする
        let mut vq2 = Vec::with_capacity(q); // edge : (edge_id, Some(r_vertex_id)) このクエリを処理するときに e_vertex_id 以降を切り離す

        let mut eid_to_q2id = vec![q; m];
        for _ in 0..q {
            let x = scan.u();
            let y = scan.u();
            vq.push(x);

            if x == 1 {
                vq1.push(y - 1);
            } else {
                eid_to_q2id[y - 1] = vq2.len();
                vq2.push((y - 1, None));
            }
        }

        /*
           エッジ削除のクエリを逆順にして、このクエリはどのグループとどのグループをくっつけるか、をDSUを使って調べる。
           x-yにエッジを追加 = (xのroot)の子として(yのroot)を追加
           x-yのエッジを削除 = (xのroot)から(yのroot)を切り離す

        */
        let mut eidq2id: Vec<(usize, usize)> = eid_to_q2id.into_iter().enumerate().collect(); // (edge_id, query2_id)
        eidq2id.sort_by_key(|x| q - x.1);

        let mut dsu = DSU::new(n);
        let mut child = vec![Vec::new(); n];
        for (edge_id, q2id) in eidq2id {
            let e = edges[edge_id];
            if !dsu.is_same(e.0, e.1) {
                let e0 = dsu.find(e.0);
                let e1 = dsu.find(e.1);
                dsu.unify(e0, e1);

                child[e0].push(e1);
                if q2id != q {
                    vq2[q2id].1 = Some(e1);
                }
            }
        }
        /*
           頂点の順番を並び替える。
           各rootからdfsで通った順にする。
           (xのroot)から(yのroot)を切り離す は、 (yのroot)の手前で分割する と同じになる
        */
        let mut pos_to_vid = Vec::with_capacity(n);
        fn dfs(vid: usize, pos_to_vid: &mut Vec<usize>, child: &Vec<Vec<usize>>) {
            pos_to_vid.push(vid);
            for &child_vid in &child[vid] {
                dfs(child_vid, pos_to_vid, child);
            }
        }

        let mut partitions = BTreeSet::new();
        partitions.insert(0);
        for i in 0..n {
            if i == dsu.find(i) {
                dfs(i, &mut pos_to_vid, &child);
            }
            partitions.insert(pos_to_vid.len());
        }

        //
        let mut vid_to_pos = vec![0; n];
        let mut v = vec![(0, 0); n];
        for pos in 0..n {
            let vid = pos_to_vid[pos];
            v[pos] = (p[vid], vid);
            vid_to_pos[vid] = pos;
        }
        let mut s = SegmentTree::new(n, (0usize, 0usize), |l, r| max(l, r), Some(&v));

        vq1.reverse();
        vq2.reverse();
        for i in 0..q {
            // for pos in 0..n {
            //     if partitions.contains(&pos) {
            //         print!("|");
            //     }
            //     let (val, vid) = s.v[s.offset + pos];
            //     print!("({} {}) ", val, vid + 1);
            // }
            // println!("");

            if vq[i] == 1 {
                let vid = vq1.pop().unwrap();
                let pos = vid_to_pos[vid];
                let lpos = partitions.range(..=pos).next_back().unwrap();
                let rpos = partitions.range(pos + 1..).next().unwrap();
                let (val2, vid2) = s.query_range(*lpos, *rpos);
                //println!("range pos{} -> {}..{}  val,vid={},{}", pos, *lpos, *rpos, val2, vid2 );
                writeln!(out, "{}", val2).ok();
                s.update(vid_to_pos[vid2], (0, vid2));
            } else {
                if let Some((_, Some(vid))) = vq2.pop() {
                    partitions.insert(vid_to_pos[vid]);
                }
            }
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