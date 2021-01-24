use std::cmp::max;
use std::cmp::min;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let _n: i64 = read_line().parse().unwrap();
    let aa = read_line()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let solver = Solver::new(aa);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    aa: Vec<u64>,
}

impl Solver {
    fn new(aa: Vec<u64>) -> Solver {
        Solver { aa: aa }
    }

    fn solve(&self) -> Vec<String> {
        let size = self.aa.len();

        // dp[l][r] = x
        let mut dp = vec![vec![std::u64::MAX; size]; size];

        let mut ans = 0;
        for l in 0..size {
            for r in 0..size {
                if l > r {
                    continue;
                }

                let x = if r == 0 { self.aa[l] } else { dp[l][r - 1] };
                dp[l][r] = min(x, self.aa[r]);
                if dp[l][r] != std::u64::MAX {
                    ans = max(ans, ((r - l + 1) as u64) * dp[l][r]);
                }
            }
        }

        let mut buf = Vec::new();
        buf.push(format!("{}", ans));
        buf
    }
}

#[test]
fn test_solve_1() {
    let solver = Solver::new(vec![2, 4, 4, 9, 4, 9]);
    assert_eq!(solver.solve(), vec!("20"));
}
