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
}

///////

#[derive(Clone, Debug, Default)]
struct Solver {
    s: BTreeSet<usize>,
    gaps: BTreeMap<usize, usize>,
}

impl Solver {
    fn insert(&mut self, pos: usize) {
        //println!("insert {}", pos);
        self.s.insert(pos);

        // 両側を得る
        let prev = self.s.range(..pos).next_back();
        let next = self.s.range(pos+1..).next();
       

        // 以前のgapを消す
        if prev != None && next != None {
            let gap = next.unwrap() - prev.unwrap();
            //println!("del gap {}", gap);
            *self.gaps.entry(gap).or_insert(0) -= 1;
        }

        // 新しくgapを追加
        if prev != None {
            let gap = pos - prev.unwrap();
            //println!("addl gap {}", gap);
            *self.gaps.entry(gap).or_insert(0) += 1;
        }
        if next != None {
            let gap = next.unwrap() - pos;
            //println!("addr gap {}", gap);
            *self.gaps.entry(gap).or_insert(0) += 1;
        }
    }
    fn remove(&mut self, pos: usize) {
        self.s.remove(&pos);
        // 両側を得る
        let prev = self.s.range(..pos).next_back();
        let next = self.s.range(pos+1..).next();
        // 以前のgapを消す
        if let Some(prev) = prev {
            let gap = pos - prev;
            *self.gaps.entry(gap).or_insert(0) -= 1;
        }
        if let Some(next) = next {
            let gap = next - pos;
            *self.gaps.entry(gap).or_insert(0) -= 1;
        }

        // 新しくgapを追加
        if prev != None && next != None {
            let gap = next.unwrap() - prev.unwrap();
            *self.gaps.entry(gap).or_insert(0) += 1;
        }
    }
    fn calc(&mut self) -> usize {
        let mut range = self.s.range(..);
        if let Some(l) = range.next() {
            if let Some(r) = range.next_back() {
                let gap;
                loop {
                    let (&max_gap,&count) = self.gaps.range(..).next_back().unwrap();
                    if count == 0 {
                        self.gaps.remove(&max_gap);
                    } else {
                        gap = max_gap;
                        break;
                    }
                }
                //println!("calc l={} r={} gap={} result={}", l, r, gap, r - l - gap);
                return r - l - gap;
            }
        }
        0
    }

    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        let n: usize = scan.cin();
        let q: usize = scan.cin();
        let p: Vec<usize> = scan.vec(n);
        let mut t = vec![0usize; q];
        let mut x = vec![0usize; q];
        for i in 0..q {
            t[i] = scan.cin();
            x[i] = scan.cin();
        }

        self.s = BTreeSet::new(); // min, max
        self.gaps = BTreeMap::new(); // gap

        for &pos in &p {
            self.insert(pos);
        }
        writeln!(out, "{}", self.calc()).ok();

        for i in 0..q {
            if t[i] == 0 {
                self.remove(x[i]);
            } else {
                self.insert(x[i]);
            }
            writeln!(out, "{}", self.calc()).ok();
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
