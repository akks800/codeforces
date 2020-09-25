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

#[derive(Clone, Debug, Default)]
struct Solver {}

impl Solver {
    fn solve(&mut self, scan: &mut Scanner, out: &mut BufWriter<Stdout>) {
        let n: usize = scan.cin();
        let s = scan.chars();
        let mut seq = vec![Vec::new(); 2]; // seq[i] : iを追加可能なsubsequenceの集合
        let mut i_belong_to = vec![0; n];
        for i in 0..n {
            let ch = (s[i] as u8 - b'0') as usize;
            if seq[ch].len() == 0 {
                let seq_no = seq[ch ^ 1].len();
                seq[ch].push(seq_no);
            }

            let seq_no = seq[ch].pop().unwrap();
            i_belong_to[i] = seq_no;
            seq[ch ^ 1].push(seq_no);
        }

        writeln!(out, "{}", seq[0].len() + seq[1].len()).ok();
        for i in i_belong_to {
            write!(out, "{} ", i + 1).ok();
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
