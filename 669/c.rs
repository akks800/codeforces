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

#[derive(Clone, Debug)]
struct Solver {}

fn solve() {
    let mut scan = Scanner::new();

    let n: usize = scan.cin();
    let mut v = vec![0; n + 5];

    let mut i = 1;
    let mut j = 2;
    let mut k = 3;
    while i <= n && j <= n {
        println!("? {} {}", i, j);
        std::io::stdout().flush().unwrap();
        let x: usize = scan.cin();
        println!("? {} {}", j, i);
        std::io::stdout().flush().unwrap();
        let y: usize = scan.cin();

        if x > y {
            // pi%pj > pj%pi ->  pi<pj && pi%pj==pi (==x) 
            v[i] = x;
            i = k;
        } else {
            // y=pj
            v[j] = y;
            j = k;
        }
        k += 1;
    }
    if i > n {
        v[j] = n;
    } else {
        v[i] = n;
    }
    print!("! ");
    for i in 1..=n {
        print!("{} ", v[i]);
    }
    println!("");
    std::io::stdout().flush().unwrap();
}

fn main() {
    std::thread::Builder::new()
        .name("big stack size".into())
        .stack_size(32 * 1024 * 1024) // 32 MBのスタックサイズ
        .spawn(|| {
            solve(); // ここで深い再帰を実行
        })
        .unwrap()
        .join()
        .unwrap();
}