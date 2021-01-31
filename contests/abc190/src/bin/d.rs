use std::collections::HashSet;

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

fn enum_divisors(n: i64) -> HashSet<i64> {
    let mut divisors = HashSet::new();
    if n == 0 {
        return divisors;
    }

    let mut i: i64 = 1;
    while i * i <= n {
        if n % i == 0 {
            divisors.insert(i);
            divisors.insert(n / i);
        }

        i += 1;
    }

    divisors
}

struct Solver {
    n: i64,
}

impl Solver {
    fn new(n: i64) -> Solver {
        Solver { n: n }
    }

    fn solve(&self) -> Vec<String> {
        let mut ans = 0;
        let dividors = enum_divisors(2 * self.n);
        for l in dividors.iter() {
            let v = (2 * self.n) / l - l + 1;
            if v % 2 == 0 {
                ans += 1;
            }
        }

        let mut buf = Vec::new();
        buf.push(format!("{}", ans));
        buf
    }
}
