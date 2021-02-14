use proconio::input;

fn main() {
    input! {
        s: String,
    }
    let solver = Solver::new(s);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    s: String,
}

impl Solver {
    fn new(s: String) -> Solver {
        Solver { s: s }
    }

    fn solve(&self) -> Vec<String> {
        let c1 = self.s.chars().nth(0).unwrap();
        let c2 = self.s.chars().nth(1).unwrap();
        let c3 = self.s.chars().nth(2).unwrap();

        let mut buf = Vec::new();
        if c1 == c2 && c1 == c3 {
            buf.push(format!("Won"));
        } else {
            buf.push(format!("Lost"));
        }
        buf
    }
}
