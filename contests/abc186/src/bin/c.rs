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
    n: u32,
}

impl Solver {
    fn new(n: u32) -> Solver {
        Solver { n: n }
    }

    fn solve(&self) -> Vec<String> {
        let mut ans = 0;
        for num in 1..=self.n {
            if format!("{}", num).contains("7") {
                continue;
            }
            if format!("{:o}", num).contains("7") {
                continue;
            }
            ans += 1;
        }

        let mut buf = Vec::new();
        buf.push(format!("{}", ans));
        buf
    }
}
