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
    fn new(n: usize, unit: T, comb: fn(T, T) -> T, vv: &Vec<T>) -> SegmentTree<T> {
        let mut x = 1;
        while x < n {
            x *= 2;
        }

        let mut v = vec![unit; x * 2];
        for i in 0..n {
            v[x + i] = vv[i];
        }
        for i in (0..x).rev() {
            v[i] = comb(v[i * 2], v[i * 2 + 1]);
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

//
// Segment Tree (Lazy)
//

/*
  nodeの区間の値は、常にval[node]とlazy[nodeとその祖先のnode]から計算した値に等しい
  val[node]は、常にその左右の子の valとlazy から計算した値に等しい
*/

#[derive(Clone)]
struct SegmentTreeLazy<T, U> {
    v: Vec<T>,
    lazy: Vec<U>,
    offset: usize,
    vvcomb: fn(T, T) -> T,
    vlcomb: fn(T, U) -> T,
    llcomb: fn(U, U) -> U,
    vunit: T,
    lunit: U,
}

#[allow(dead_code)]
impl<T: Copy, U: Copy + PartialEq> SegmentTreeLazy<T, U> {
    fn new(
        n: usize,
        vunit: T,
        lunit: U,
        vvcomb: fn(T, T) -> T,
        vlcomb: fn(T, U) -> T,
        llcomb: fn(U, U) -> U,
        vv: &Vec<T>,
    ) -> SegmentTreeLazy<T, U> {
        let mut x = 1;
        while x < n {
            x *= 2;
        }

        let mut s = SegmentTreeLazy {
            v: vec![vunit; x * 2],
            lazy: vec![lunit; x * 2],
            offset: x,
            vunit,
            lunit,
            vvcomb,
            vlcomb,
            llcomb,
        };
        s.set_all(&vv);
        s
    }

    // set: 値を書き換える
    fn set_all(&mut self, v: &Vec<T>) {
        for i in 0..v.len() {
            self.v[self.offset + i] = v[i];
        }
        for i in (0..self.offset).rev() {
            let lval = self.v[i * 2];
            let rval = self.v[i * 2 + 1];
            self.v[i] = (self.vvcomb)(lval, rval);
        }
        for i in 0..self.lazy.len() {
            self.lazy[i] = self.lunit;
        }
    }
    // このノードのlazyを解消する
    fn eval(&mut self, node: usize) {
        let l = self.lazy[node];
        if l != self.lunit {
            if node < self.offset {
                self.lazy[node * 2 + 0] = (self.llcomb)(self.lazy[node * 2 + 0], l);
                self.lazy[node * 2 + 1] = (self.llcomb)(self.lazy[node * 2 + 1], l);
            }
            self.v[node] = (self.vlcomb)(self.v[node], self.lazy[node]);
            self.lazy[node] = self.lunit;
        }
    }

    fn update_range(&mut self, begin: usize, end: usize, val: U) {
        self.update_range_sub(begin, end, 1, 0, self.offset, val);
    }

    // update : U を適用する
    // node のlazyは解消する。親でvvcombを使うので。
    fn update_range_sub(
        &mut self,
        begin: usize,
        end: usize,
        node: usize,
        l: usize,
        r: usize,
        val: U,
    ) {
        if end <= l || r <= begin {
            self.eval(node);
        } else if begin <= l && r <= end {
            //self.eval(node);
            self.lazy[node] = (self.llcomb)(self.lazy[node], val);
            self.eval(node);
        } else {
            self.eval(node);
            self.update_range_sub(begin, end, node * 2, l, (l + r) / 2, val);
            self.update_range_sub(begin, end, node * 2 + 1, (l + r) / 2, r, val);
            self.v[node] = (self.vvcomb)(self.v[node * 2 + 0], self.v[node * 2 + 1]);
        }
    }

    fn query_range(&mut self, begin: usize, end: usize) -> T {
        self.query_range_sub(begin, end, 1, 0, self.offset)
    }

    // v[node], lazy[node] は 区間 l .. r-1 の計算結果を持っている
    fn query_range_sub(&mut self, begin: usize, end: usize, node: usize, l: usize, r: usize) -> T {
        if end <= l || r <= begin {
            return self.vunit.clone();
        } else if begin <= l && r <= end {
            self.eval(node);
            return self.v[node].clone();
        } else {
            self.eval(node);
            let lval = self.query_range_sub(begin, end, node * 2, l, (l + r) / 2);
            let rval = self.query_range_sub(begin, end, node * 2 + 1, (l + r) / 2, r);
            return (self.vvcomb)(lval, rval);
        }
    }
}

const MAX_UVAL: usize = 300_005;
const MAX_IVAL: i32 = 300_005;

// コンテスト終了後に作成
fn main() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());

    let n: usize = scan.cin();
    let q: usize = scan.cin();
    let a: Vec<i32> = scan.vec(n);

    // v[i] : これより左の要素を何回除去したらこれが除去できるようになるか
    let mut v = Vec::with_capacity(n);
    for i in 0..n {
        let val = i as i32 - (a[i] - 1);
        v.push((i, if val < 0 { MAX_IVAL * 2 } else { val }));
    }

    // 除去の順番を計算する
    let mut pos_to_order = vec![MAX_UVAL; n];
    let mut order_to_pos = Vec::with_capacity(n);
    {
        type PosVal = (usize, i32); // position, value
        const MAX_T: PosVal = (MAX_UVAL, MAX_IVAL);
        let vvcomb = |l: PosVal, r: PosVal| {
            // 小さい値(同じなら右側)を返す
            if l.1 < r.1 {
                l
            } else {
                r
            }
        };
        let vlcomb = |val: PosVal, add| (val.0, val.1 + add);
        let llcomb = |cur, add| cur + add;
        let mut s = SegmentTreeLazy::new(n, MAX_T, 0i32, vvcomb, vlcomb, llcomb, &v);
        // segment tree -> remove_val, remove_order
        for i in 0..n {
            let (pos, val) = s.query_range(0, n);
            assert!(0 <= val);
            if val != 0 {
                break;
            } else {
                pos_to_order[pos] = i;
                order_to_pos.push(pos);
            }
            s.update_range(pos, pos + 1, MAX_IVAL * 2);
            s.update_range(pos + 1, n, -1);
        }
    }

    //println!("v: {:?}", v);
    //println!("po: {:?}", pos_to_order);
    //println!("op: {:?}", order_to_pos);

    // query
    let mut qq = Vec::with_capacity(q);
    for i in 0..q {
        let x: usize = scan.cin();
        let y: usize = scan.cin();
        qq.push((i, x, n - y)); // fix first x and last y elements
    }
    qq.sort_by_key(|x| MAX_UVAL - x.1);

    let v = vec![1; n];
    let comb = |a, b| a + b;
    let mut s = SegmentTree::new(n, 0i32, comb, &v); // 位置 → 残っている要素数
    let mut ans = vec![0; q];
    {
        // orderの順にsegment tree から除去していき、 posが除去できない(pos < q.l) ときに q.l <= pos < q.r で削除済みの要素数を計算する
        let mut order = 0;
        for (i, l, r) in qq {
            //println!("qq: i={} l={} r={}", i, l, r);
            while order < order_to_pos.len() && l <= order_to_pos[order] {
                let pos = order_to_pos[order];
                s.update(pos, 0);
                order += 1;
                //println!("remove {:?}", pos);
            }
            ans[i] = (r - l) - s.query_range(l, r) as usize;
            //println!("ans {} = {}", i, ans[i]);
        }
    }

    for i in 0..q {
        writeln!(out, "{}", ans[i]).ok();
    }
}
