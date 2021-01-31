fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let n = read_line().parse().unwrap();
    let solver = Solver::new(n);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    n: i64,
}

impl Solver {
    fn new(n: i64) -> Solver {
        Solver { n: n }
    }

    fn solve(&self) -> Vec<String> {
        let mut ok_idx: i64 = -1;
        let mut ng_idx: i64 = self.n + 1;
        while (ok_idx - ng_idx).abs() > 1 {
            let x = (ok_idx + ng_idx) / 2;
            let sum = (x as i128) * ((x as i128) + 1) / 2;
            if sum <= (self.n + 1) as i128 {
                ok_idx = x;
            } else {
                ng_idx = x;
            }
        }

        let mut buf = Vec::new();
        buf.push(format!("{}", self.n - ok_idx + 1));
        buf
    }
}
