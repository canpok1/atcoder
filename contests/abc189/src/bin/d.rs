fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let n = read_line().parse().unwrap();
    let mut ss = Vec::new();
    (0..n).for_each(|_| {
        let s = read_line().parse().unwrap();
        ss.push(s);
    });
    let solver = Solver::new(ss);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    ss: Vec<String>,
}

impl Solver {
    fn new(ss: Vec<String>) -> Solver {
        Solver { ss: ss }
    }

    fn solve(&self) -> Vec<String> {
        // dp[i] : x0〜xiがTrueになるパターン数
        let mut dp = vec![std::u64::MAX; self.ss.len() + 1];
        (0..=self.ss.len()).for_each(|i| {
            dp[i] = if i == 0 {
                1
            } else if self.ss[i - 1] == "AND" {
                dp[i - 1]
            } else {
                dp[i - 1] + 2_u64.pow((i) as u32)
            };
        });

        let mut buf = Vec::new();
        buf.push(format!("{}", dp.last().unwrap()));
        buf
    }
}

#[test]
fn test_solve_1() {
    let solver = Solver::new(vec!["AND".to_owned()]);
    assert_eq!(solver.solve(), vec!("1"));
}

#[test]
fn test_solve_2() {
    let solver = Solver::new(vec!["OR".to_owned()]);
    assert_eq!(solver.solve(), vec!("3"));
}

#[test]
fn test_solve_3() {
    let solver = Solver::new(vec!["AND".to_owned(), "OR".to_owned()]);
    assert_eq!(solver.solve(), vec!("5"));
}
