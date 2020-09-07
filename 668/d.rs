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

fn calc_dist(e: &Vec<Vec<usize>>, u: usize) -> (usize, usize, Vec<usize>) {
    let n = e.len();
    let mut q = BinaryHeap::new();
    const MAX_N: usize = 200005;
    let mut dist = vec![MAX_N; n];
    dist[u] = 0;
    q.push((0, u));

    while let Some(p) = q.pop() {
        for &i in &e[p.1] {
            if dist[i] == MAX_N {
                dist[i] = p.0 + 1;
                q.push((p.0 + 1, i));
            }
        }
    }
    let mut max_dist_pos = u;
    let mut max_dist = 0;
    for i in 0..n {
        if max_dist < dist[i] {
            max_dist = dist[i];
            max_dist_pos = i;
        }
    }
    (max_dist, max_dist_pos, dist)
}

fn main() {
    let mut scan = Scanner::new();
    let out = &mut BufWriter::new(stdout());
    let testcase: usize = scan.cin();
    for _ in 0..testcase {
        let n: usize = scan.cin();
        let a: usize = scan.cin::<usize>() - 1;
        let b: usize = scan.cin::<usize>() - 1;
        let da: usize = scan.cin();
        let db: usize = scan.cin();
        let mut e = vec![Vec::new(); n];
        for _ in 0..n - 1 {
            let u: usize = scan.cin();
            let v: usize = scan.cin();
            e[u - 1].push(v - 1);
            e[v - 1].push(u - 1);
        }

        //
        let mut alice = false;
        let (_, far_1, dist) = calc_dist(&e, a);
        //println!("{} {} {:?}", a, b, dist);
        if dist[b] <= da {
            alice = true;
        } else {
            let (max_dist, _, _) = calc_dist(&e, far_1);
            if max_dist <= da * 2 {
                alice = true;
            } else if db < da * 2 + 1 {
                alice = true;
            }
        }

        /*
          if distance(a,b) <= da {
              a win
          } else if distance of graph <= da*2 {
              a win
          } else if db < da*2+1 {
              a win
          } else {
              b win
          }
        */

        writeln!(out, "{}", if alice { "Alice" } else { "Bob" }).ok();
    }
}
