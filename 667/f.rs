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


// chmax https://qiita.com/maguro_tuna/items/fab200fdc1efde1612e7
macro_rules! max {
    ($a:expr $(,)*) => {{
        $a
    }};
    ($a:expr, $b:expr $(,)*) => {{
        std::cmp::max($a, $b)
    }};
    ($a:expr, $($rest:expr),+ $(,)*) => {{
        std::cmp::max($a, max!($($rest),+))
    }};
}

macro_rules! chmax {
    ($base:expr, $($cmps:expr),+ $(,)*) => {{
        let cmp_max = max!($($cmps),+);
        if $base < cmp_max {
            $base = cmp_max;
            true
        } else {
            false
        }
    }};
}

// Editorialを見て作成
fn main() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());
    let n: usize = scan.cin();
    let max_k: usize = scan.cin();
    let s = scan.chars();
    let t = scan.chars();

    let v: Vec<usize> = s
        .iter()
        .map(|&c| {
            if c == t[0] {
                0
            } else if c == t[1] {
                1
            } else {
                2
            }
        })
        .collect();

    if t[0] == t[1] {
        let ct = v.iter().filter(|&x| *x == 0).count();
        let x = min(ct + max_k, n);
        writeln!(out, "{}", x * (x - 1) / 2).ok();
        return;
    }
    // dp[i][k][ct] : [0,i) の範囲で、k回操作して、t[0]の文字がct個ある場合、この範囲にsubsequenceを何通り作れるか
    let mut dp = vec![vec![vec![-1000000i32; n + 2]; n + 2]; n + 2];
    dp[0][0][0] = 0;

    for i in 0..n {
        for k in 0..=max_k {
            for ct in 0..=i {
                let c = ct as i32;
                if v[i] == 0 {
                    //               op       ->0                 ->1
                    chmax!(dp[i + 1][k + 0][ct + 1], dp[i][k][ct] + 0); // 0->0
                    chmax!(dp[i + 1][k + 1][ct + 0], dp[i][k][ct] + c); // 0->1
                } else if v[i] == 1 {
                    chmax!(dp[i + 1][k + 1][ct + 1], dp[i][k][ct] + 0); // 1->0
                    chmax!(dp[i + 1][k + 0][ct + 0], dp[i][k][ct] + c); // 1->1
                } else if v[i] == 2 {
                    chmax!(dp[i + 1][k + 1][ct + 1], dp[i][k][ct] + 0); // 2->0
                    chmax!(dp[i + 1][k + 1][ct + 0], dp[i][k][ct] + c); // 2->1
                    chmax!(dp[i + 1][k + 0][ct + 0], dp[i][k][ct] + 0); // 2->2
                }
            }
        }
    }

    let mut ans = 0;
    for k in 0..=max_k {
        //print!(" k={} : ", k);
        for ct in 0..=n {
            //print!("{} ", dp[i][k][ct]);
            chmax!(ans, dp[n][k][ct]);
        }
        //println!(" ");
    }

    writeln!(out, "{} ", ans).ok();
}
