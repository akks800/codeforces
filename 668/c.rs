//use std::cmp::*;
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

fn main() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());
    let testcase: usize = scan.cin();
    for _ in 0..testcase {
        let n: usize = scan.cin();
        let k: usize = scan.cin();
        let mut a = scan.chars();
        let mut ok = true;
        for i in 0..k {
            let mut ch = '?';
            for j in (i..n).step_by(k) {
                if ch == '?' {
                    ch = a[j];
                } else if a[j] == '?' {
                    a[j] = ch;
                } else {
                    if ch != a[j] {
                        ok = false;
                        break;
                    }
                }
            }
        }
        //let s:String = a.iter().collect();
        //println!("{}", s);

        let mut c0 = 0;
        let mut c1 = 0;
        for i in n - k..n {
            if a[i] == '0' {
                c0 += 1;
            }
            if a[i] == '1' {
                c1 += 1;
            }
        }
        if c0 > k / 2 || c1 > k / 2 {
            ok = false;
        }

        writeln!(out, "{}", if ok { "YES" } else { "NO" }).ok();
    }
}
