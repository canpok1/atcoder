use std::collections::HashSet;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let k = iter.next().unwrap().parse().unwrap();

    // tt[i][j] = Tij
    let mut tt: Vec<Vec<u64>> = Vec::new();
    (0..n).for_each(|_| {
        let t = read_line()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        tt.push(t);
    });

    let mut solver = Solver::new(k, tt);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    n: usize,
    k: u64,
    tt: Vec<Vec<u64>>,
    visited: HashSet<usize>,
    ans: u64,
}

impl Solver {
    fn new(k: u64, tt: Vec<Vec<u64>>) -> Solver {
        Solver {
            n: tt.len(),
            k: k,
            tt: tt,
            visited: HashSet::new(),
            ans: 0,
        }
    }

    fn solve(&mut self) -> Vec<String> {
        self.visited.clear();
        self.ans = 0;
        self.solv_rec(0, 0, 0);
        let mut buf = Vec::new();
        buf.push(format!("{}", self.ans));
        buf
    }

    fn solv_rec(&mut self, current: usize, next: usize, cost: u64) {
        let cost = cost + self.tt[current][next];
        self.visited.insert(next);

        if self.visited.len() == self.n {
            let cost = cost + self.tt[next][0];
            if cost == self.k {
                self.ans += 1;
            }
            return;
        }

        for city in 0..self.n {
            if self.visited.contains(&city) {
                continue;
            }
            self.solv_rec(next, city, cost);
            self.visited.remove(&city);
        }
    }
}
