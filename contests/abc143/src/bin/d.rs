fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let n = read_line().parse().unwrap();
    let ll = read_line()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let mut solver = Solver::new(n, ll);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    n: usize,
    ll: Vec<i64>,
}

impl Solver {
    fn new(n: usize, ll: Vec<i64>) -> Solver {
        Solver { n: n, ll: ll }
    }

    fn solve(&mut self) -> Vec<String> {
        self.ll.sort();

        let mut ans = 0;
        for i1 in (2..self.n).rev() {
            let a = self.ll[i1];
            for i2 in (1..i1).rev() {
                if i2 == i1 {
                    continue;
                }
                let b = self.ll[i2];
                let diff = (a - b).abs();
                match self.ll.binary_search(&diff) {
                    Ok(i3) => {
                        if i2 > i3 + 1 {
                            ans += i2 - (i3 + 1);
                        }
                    }
                    Err(i3) => {
                        if i2 > i3 {
                            ans += i2 - i3;
                        }
                    }
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
    let n = 2000;
    let ll = vec![1; 2000];
    let mut solver = Solver::new(n, ll);
    assert_eq!(solver.solve(), vec!("1331334000"));
}
