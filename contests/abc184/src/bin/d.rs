use std::collections::HashMap;

#[allow(dead_code)]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let a = iter.next().unwrap().parse().unwrap();
    let b = iter.next().unwrap().parse().unwrap();
    let c = iter.next().unwrap().parse().unwrap();

    let solver = Solver::new(a, b, c);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    a: i64,
    b: i64,
    c: i64,
}

impl Solver {
    fn new(a: i64, b: i64, c: i64) -> Solver {
        Solver { a: a, b: b, c: c }
    }

    fn solve(&self) -> Vec<String> {
        let mut memo: HashMap<(i64, i64, i64), f64> = HashMap::new();
        let ans = self.solve_by(self.a, self.b, self.c, &mut memo);

        let mut buf = Vec::new();
        buf.push(format!("{:.6}", ans));
        buf
    }

    fn solve_by(&self, x: i64, y: i64, z: i64, memo: &mut HashMap<(i64, i64, i64), f64>) -> f64 {
        if x == 100 || y == 100 || z == 100 {
            return 0.0;
        }
        if memo.contains_key(&(x, y, z)) {
            return memo[&(x, y, z)];
        }

        let xf = x as f64;
        let yf = y as f64;
        let zf = z as f64;
        let ans = if (x == 0 && y == 0) || (x == 0 && z == 0) || (y == 0 && z == 0) {
            (100 - x - y - z) as f64
        } else {
            let ans1 = if x == 0 {
                0.0
            } else {
                xf / (xf + yf + zf) * (self.solve_by(x + 1, y, z, memo) + 1.0)
            };
            let ans2 = if y == 0 {
                0.0
            } else {
                yf / (xf + yf + zf) * (self.solve_by(x, y + 1, z, memo) + 1.0)
            };
            let ans3 = if z == 0 {
                0.0
            } else {
                zf / (xf + yf + zf) * (self.solve_by(x, y, z + 1, memo) + 1.0)
            };
            ans1 + ans2 + ans3
        };

        memo.insert((x, y, z), ans);

        ans
    }
}
