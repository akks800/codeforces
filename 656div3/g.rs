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

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        //1385g
        let n = scan.u();
        let mut v = vec![vec![0;2];n];
        let mut val_to_pos = vec![Vec::with_capacity(2); n];
        for i in 0..n {
            let a = scan.u() - 1;
            v[i][0] = a;
            val_to_pos[a].push((i, 0));
        }
        for i in 0..n {
            let b = scan.u() - 1;
            v[i][1] = b;
            val_to_pos[b].push((i, 1));
        }

        if val_to_pos.iter().any(|v| (*v).len() != 2) {
            writeln!(out, "-1").ok();
            return;
        }

        let mut ans = Vec::new();
        let mut fixed = vec![false; n];
        for i in 0..n {
            /* 列0から順に、列を固定→他の列も連鎖的に固定 を行う */
            if !fixed[i] {
                let mut flip = Vec::new();
                let mut noflip = Vec::new();
                let apos0 = (i, 0);
                let mut apos = apos0;
                loop {
                    let a = v[apos.0][apos.1];
                    //println!( "apos={}.{}", apos.0, apos.1);
                    assert!(!fixed[apos.0]);
                    fixed[apos.0] = true;

                    let aapos = if val_to_pos[a][0] == apos {
                        val_to_pos[a][1]
                    } else {
                        val_to_pos[a][0]
                    };
                    //println!( "aapos={}.{}", aapos.0, aapos.1);

                    if aapos.1 == 1 {
                        noflip.push(aapos.0);
                        apos = (aapos.0, 0);
                    } else {
                        flip.push(aapos.0);
                        apos = (aapos.0, 1);
                    }
                    if apos == apos0 {
                        break;
                    }
                }

                if flip.len() > noflip.len() {
                    ans.append(&mut noflip);
                } else {
                    ans.append(&mut flip);
                }
            }
        }

        writeln!(out, "{}", ans.len()).ok();

        for i in ans {
            write!(out, "{} ", i+1).ok();
        }
        writeln!(out, "").ok();
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
