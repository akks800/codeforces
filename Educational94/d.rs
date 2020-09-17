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
    const MAX_N: usize = 3001;

    for _ in 0..testcase {
        let n = scan.cin();
        let a: Vec<usize> = scan.vec(n);
        let mut color_h = vec![0; MAX_N];
        let mut color_pos = vec![MAX_N; MAX_N]; // color -> pos
        let mut pos_nextpos = vec![MAX_N; MAX_N]; // pos -> nextpos
        for i in (0..n).rev() {
            let color = a[i] - 1;
            pos_nextpos[i] = color_pos[color];
            color_pos[color] = i;
            color_h[color] += 1;
        }
        /*
          for ( pos = color_pos[color]; pos != MAX_N; pos = pos_nextpos[pos] ) ... で同じ色をたどれる
        */

        let mut ans = 0u64;
        for col1 in 0..n {
            if color_h[col1] < 2 {
                continue;
            }
            for col2 in 0..n {
                if col1 == col2 {
                    let x = color_h[col1] as u64;
                    if x >= 4 {
                        ans += x * (x - 1) * (x - 2) * (x - 3) / 24;
                    }
                    continue;
                }
                if color_h[col2] < 2 {
                    continue;
                }
                let mut pos1 = color_pos[col1];
                let mut pos2 = color_pos[col2];
                while pos2 > pos1 {
                    pos1 = pos_nextpos[pos1];
                }
                let mut val1 = 0; // pos1より手前にあるpos2の数 // a
                let mut val2 = 0; // pos2より手前にあるpos1のval1の和 // ab
                let mut val1s = 0; // pos1より手前にあるpos2のval2の和 // aba
                let mut val2s = 0; // abab
                while pos1 != MAX_N {
                    while pos2 < pos1 {
                        val1 += 1;
                        val1s += val2;
                        pos2 = pos_nextpos[pos2];
                        //println!("a {} {} {} {}", val1, val2, val1s, val2s);
                    }
                    while pos1 < pos2 {
                        val2 += val1;
                        val2s += val1s;
                        pos1 = pos_nextpos[pos1];
                        //println!("b {} {} {} {}", val1, val2, val1s, val2s);
                    }
                }
                /*
                                let mut dp = vec![vec![0; 4]; n+1];
                                let mut v = Vec::new();
                                for i in 0..n {
                                    if a[i]-1 == col1 {
                                        v.push(1);
                                    }
                                    if a[i]-1 == col2 {
                                        v.push(2);
                                    }
                                }
                                for i in 1..=v.len() {
                                    if v[i - 1] == 2 {
                                        dp[i][0] = dp[i - 1][0] + 1;
                                        dp[i][1] = dp[i - 1][1];
                                        dp[i][2] = dp[i - 1][2] + dp[i - 1][1];
                                        dp[i][3] = dp[i - 1][3];
                                    } else {
                                        dp[i][0] = dp[i - 1][0];
                                        dp[i][1] = dp[i - 1][1] + dp[i - 1][0];
                                        dp[i][2] = dp[i - 1][2];
                                        dp[i][3] = dp[i - 1][3] + dp[i - 1][2];
                                    }
                                }
                                println!("* {} {}", val2s, dp[v.len()][3]);
                */

                ans += val2s;
            }
        }
        writeln!(out, "{}", ans).ok();
    }
}
