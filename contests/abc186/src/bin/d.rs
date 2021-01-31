fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let _n: usize = read_line().parse().unwrap();
    let aa = read_line()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    let mut solver = Solver::new(aa);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    n: usize,
    aa: Vec<i32>,
}

impl Solver {
    fn new(aa: Vec<i32>) -> Solver {
        Solver {
            n: aa.len(),
            aa: aa,
        }
    }

    fn solve(&mut self) -> Vec<String> {
        self.aa.sort();

        // 累積和
        let mut sums: Vec<i64> = Vec::new();
        for a in self.aa.iter() {
            let sum = match sums.last() {
                Some(n) => n + (*a as i64),
                None => *a as i64,
            };
            sums.push(sum);
        }

        let mut ans = 0;
        for (i, a) in self.aa.iter().enumerate() {
            ans +=
                (sums[self.n - 1] - sums[i]) - ((self.n as i64) - (i as i64) - 1) * ((*a) as i64);
        }

        let mut buf = Vec::new();
        buf.push(format!("{}", ans));
        buf
    }
}
