mod math;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let solver = Solver::new(read_line());
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
        let mut buf = Vec::new();
        buf.push(format!("{}", self.s));
        buf
    }
}

#[test]
fn test_solve_1() {
    let solver = Solver::new("1".to_owned());
    assert_eq!(solver.solve(), vec!("1"));
}
