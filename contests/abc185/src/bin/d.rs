use std::cmp::min;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();
    let a = read_line()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let mut solver = Solver::new(n, m, a);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    n: usize,
    m: usize,
    aa: Vec<usize>,
}

impl Solver {
    fn new(n: usize, m: usize, aa: Vec<usize>) -> Solver {
        Solver { n: n, m: m, aa: aa }
    }

    fn solve(&mut self) -> Vec<String> {
        if self.m == 0 {
            let mut buf = Vec::new();
            buf.push(format!("{}", 1));
            return buf;
        }
        if self.n == self.m {
            let mut buf = Vec::new();
            buf.push(format!("{}", 0));
            return buf;
        }

        self.aa.sort();

        let mut dp: Vec<usize> = vec![self.n; self.m];
        let mut whites: Vec<usize> = Vec::new();
        let mut before_a: usize = 0;
        for (i, a) in self.aa.iter().enumerate() {
            let white = a - before_a - 1;
            if white == 0 {
                dp[i] = if whites.len() == 0 {
                    self.n - a
                } else {
                    dp[i - 1]
                };
            } else {
                dp[i] = if i == 0 { white } else { min(dp[i - 1], white) };
                whites.push(white);
            }
            before_a = *a;
        }
        let white = (self.n + 1) - before_a - 1;
        if white > 0 {
            whites.push(white);
        }

        let k = match dp.last() {
            Some(n) => *n,
            None => self.n,
        };
        let mut ans = 0;
        for w in whites {
            ans += w / k;
            if w % k != 0 {
                ans += 1;
            }
        }

        let mut buf = Vec::new();
        buf.push(format!("{}", ans));
        buf
    }
}
