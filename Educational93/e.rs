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
    fn vecu(&mut self, n: usize) -> Vec<usize> {
        self.vec(n)
    }
}

///////

#[allow(unused_macros)]
macro_rules! chmin {
    ($dst:expr, $src:expr) => {{
        let b = $dst > $src;
        if b {
            $dst = $src;
        }
        b
    }};
}

#[allow(unused_macros)]
macro_rules! chmax {
    ($dst:expr, $src:expr) => {{
        let b = $dst < $src;
        if b {
            $dst = $src;
        }
        b
    }};
}

#[derive(Clone, Debug, Default)]
struct Solver {
    sh: BTreeSet<(i64, usize)>,
    sl: BTreeSet<(i64, usize)>,
    sumh: i64,
    suml: i64,
    nlh: usize, // #lightning in sh
}

impl Solver {
    // move sh to sl
    fn hl(&mut self) {
        assert!(!self.sh.is_empty());
        if let Some(&(d, tp)) = self.sh.range(..).next() {
            self.sh.remove(&(d, tp));
            self.sl.insert((d, tp));
            self.sumh -= d;
            self.suml += d;
            if tp == 1 {
                self.nlh -= 1;
            }
        }
    }
    // move sl to sh
    fn lh(&mut self) {
        assert!(!self.sl.is_empty());
        if let Some(&(d, tp)) = self.sl.range(..).next_back() {
            self.sl.remove(&(d, tp));
            self.sh.insert((d, tp));
            self.suml -= d;
            self.sumh += d;
            if tp == 1 {
                self.nlh += 1;
            }
        }
    }
    fn learn(&mut self, tp: usize, d: i64) {
        if tp == 0 {
            self.sl.insert((d, tp));
            self.suml += d;
            self.lh();
            self.hl();
        } else {
            self.sh.insert((d, tp));
            self.sumh += d;
            self.nlh += 1;
            self.hl();
            self.lh();
        }
    }

    fn forget(&mut self, tp: usize, d: i64) {
        if self.sh.contains(&(d, tp)) {
            self.sh.remove(&(d, tp));
            self.sumh -= d;
            if tp == 1 {
                self.nlh -= 1;
            } else {
                self.lh();
            }
        } else if self.sl.contains(&(d, tp)) {
            self.sl.remove(&(d, tp));
            self.suml -= d;
            if tp == 1 {
                self.hl();
            }
        } else {
            panic!();
        }
    }
    fn calc(&mut self) -> i64 {
        let mut ans = self.sumh * 2 + self.suml;
        if self.sh.len() == self.nlh {
            if let Some(&(d, _tp)) = self.sh.range(..).next() {
                ans -= d;
                if let Some(&(d, _tp)) = self.sl.range(..).next_back() {
                    ans += d;
                }
            }
        }
        ans
    }

    #[allow(dead_code)]
    fn dump(&mut self) {
        println!("---- nlh={}", self.nlh);
        for &(d, tp) in self.sh.iter() {
            print!("{}/{} ", d, tp);
        }
        println!(" h total={}", self.sumh);
        for &(d, tp) in self.sl.iter() {
            print!("{}/{} ", d, tp);
        }
        println!(" l total={}", self.suml);
        let nlh = self.sh.iter().filter(|x| x.1 == 1).count();
        let nll = self.sl.iter().filter(|x| x.1 == 1).count();
        assert!(nlh == self.nlh);
        assert!(nlh + nll == self.sh.len());
    }

    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        let n: usize = scan.cin();

        for _ in 0..n {
            let tp: usize = scan.cin();
            let d: i64 = scan.cin();

            /*
                nf個の fire spells と nl個の lightning spells を知っている
                上位nl 個の呪文が全て lightning の場合
                  上位 nl 個の内、nl-1個を2倍に
                  下位 nf 個の内、1個を2倍に
                それ以外
                  上位 nl 個の内、nl 個を2倍に
                  下位 nf 個の内、0 個を2倍に
            */

            if d < 0 {
                self.forget(tp, -d as i64);
            } else {
                self.learn(tp, d as i64);
            }
            //self.dump();
            writeln!(out, "{} ", self.calc()).ok();
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
