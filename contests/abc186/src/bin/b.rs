use std::cmp::min;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let h = iter.next().unwrap().parse().unwrap();
    let w = iter.next().unwrap().parse().unwrap();
    let mut aa: Vec<Vec<u32>> = Vec::new();
    (0..h).for_each(|_| {
        let a = read_line()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        aa.push(a);
    });

    let solver = Solver::new(h, w, aa);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    h: u32,
    w: u32,
    aa: Vec<Vec<u32>>,
}

impl Solver {
    fn new(h: u32, w: u32, aa: Vec<Vec<u32>>) -> Solver {
        Solver { h: h, w: w, aa: aa }
    }

    fn solve(&self) -> Vec<String> {
        let mut min_a: u32 = std::u32::MAX;
        let mut total: u32 = 0;
        for aa in &self.aa {
            for a in aa {
                min_a = min(min_a, *a);
                total += a;
            }
        }

        let mut buf = Vec::new();
        buf.push(format!("{}", total - min_a * self.h * self.w));
        buf
    }
}
