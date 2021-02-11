use num_integer::Integer;

#[allow(dead_code)]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

const N_I: i64 = 10_000;
const N_F: f64 = 10_000.0;

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let x = iter.next().unwrap().parse().unwrap();
    let y = iter.next().unwrap().parse().unwrap();
    let r = iter.next().unwrap().parse().unwrap();
    let solver = Solver::new(x, y, r);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    x: i64,
    y: i64,
    r: i64,
}
impl Solver {
    fn new(x: f64, y: f64, r: f64) -> Solver {
        let x = (x * N_F).round() as i64;
        let y = (y * N_F).round() as i64;
        let r = (r * N_F).round() as i64;

        Solver { x: x, y: y, r: r }
    }
    fn solve(&self) -> Vec<String> {
        let xi_min = self.x - self.r;
        let xi_max = self.x + self.r;
        let mut xi = xi_min.div_ceil(&N_I) * N_I;

        let mut ans: i64 = 0;
        while xi_min <= xi && xi <= xi_max {
            let sqrt = ((self.r.pow(2) - (xi - self.x).pow(2)) as f64).sqrt() as i64;
            let top = (self.y + sqrt).div_floor(&N_I);
            let bottom = (self.y - sqrt).div_ceil(&N_I);

            if top >= bottom {
                ans += top - bottom + 1;
            }
            xi += N_I;
        }
        let mut buf = Vec::new();
        buf.push(format!("{}", ans));
        buf
    }
}

#[test]
fn test_1() {
    let solver = Solver::new(0.0, 0.0, 0.0);
    assert_eq!(solver.solve(), vec!("1"));
}

#[test]
fn test_2() {
    let solver = Solver::new(0.1, 0.1, 0.0);
    assert_eq!(solver.solve(), vec!("0"));
}

#[test]
fn test_3() {
    let solver = Solver::new(0.1, 0.1, 0.2);
    assert_eq!(solver.solve(), vec!("1"));
}

#[test]
fn test_4() {
    let solver = Solver::new(100.0, 100.0, 1.0);
    assert_eq!(solver.solve(), vec!("5"));
}

#[test]
fn test_5() {
    let solver = Solver::new(0.0001, 0.0001, 0.0001);
    assert_eq!(solver.solve(), vec!("0"));
}

#[test]
fn test_6() {
    let solver = Solver::new(0.0, 0.9999, 0.0001);
    assert_eq!(solver.solve(), vec!("1"));
}
