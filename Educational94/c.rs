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
        let s = scan.chars();
        let n = s.len();
        let x: usize = scan.cin();
        let mut v: Vec<char> = vec!['0'; n];
        const ERR: char = '9';
        for i in 0..n {
            let left = if x <= i { s[i - x] } else { ERR };
            let right = if i + x < n { s[i + x] } else { ERR };

            if left == ERR && right == ERR {
                v[i] = '0';
            } else if left != ERR && right != ERR {
                if left == right {
                    v[i] = left;
                } else {
                    v[i] = '0';
                }
            } else {
                if left == ERR {
                    v[i] = right;
                } else {
                    v[i] = left;
                }
            }
        }

        let mut ok = true;
        for i in 0..n {
            let left = if x <= i { v[i - x] } else { ERR };
            let right = if i + x < n { v[i + x] } else { ERR };
            if left == '1' || right == '1' {
                if s[i] == '0' {
                    ok = false;
                    break;
                }
            } else {
                if s[i] == '1' {
                    ok = false;
                    break;
                }
            }
        }
        if ok {
            let s :String = v.iter().collect();
            writeln!(out, "{}", s).ok();

        } else {
            writeln!(out, "{}", -1).ok();
        }
    }

}