use std::cmp::max;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

struct Condition {
    a: usize,
    b: usize,
}
struct Human {
    c: usize,
    d: usize,
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let m = iter.next().unwrap().parse().unwrap();
    let mut conditions: Vec<Condition> = Vec::new();
    (0..m).for_each(|_| {
        let stdin = read_line();
        let mut iter = stdin.split_whitespace();
        let a: usize = iter.next().unwrap().parse().unwrap();
        let b: usize = iter.next().unwrap().parse().unwrap();
        conditions.push(Condition { a: a - 1, b: b - 1 });
    });

    let k = read_line().parse().unwrap();
    let mut humans: Vec<Human> = Vec::new();
    (0..k).for_each(|_| {
        let stdin = read_line();
        let mut iter = stdin.split_whitespace();
        let c: usize = iter.next().unwrap().parse().unwrap();
        let d: usize = iter.next().unwrap().parse().unwrap();
        humans.push(Human { c: c - 1, d: d - 1 });
    });

    let solver = Solver::new(n, conditions, humans);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    n: usize,
    conditions: Vec<Condition>,
    humans: Vec<Human>,
}

impl Solver {
    fn new(n: usize, cc: Vec<Condition>, hh: Vec<Human>) -> Solver {
        Solver {
            n: n,
            conditions: cc,
            humans: hh,
        }
    }

    fn solve(&self) -> Vec<String> {
        let mut dishs: Vec<bool> = vec![false; self.n];
        let mut buf = Vec::new();
        buf.push(format!("{}", self.put(0, &mut dishs)));
        buf
    }

    fn put(&self, human_index: usize, dishs: &mut Vec<bool>) -> usize {
        if human_index >= self.humans.len() {
            let mut ans = 0;
            for c in &self.conditions {
                if dishs[c.a] && dishs[c.b] {
                    ans += 1;
                }
            }
            return ans;
        }

        let h = &self.humans[human_index];

        let before = dishs[h.c];
        dishs[h.c] = true;
        let mut ans = self.put(human_index + 1, dishs);
        dishs[h.c] = before;

        let before = dishs[h.d];
        dishs[h.d] = true;
        ans = max(ans, self.put(human_index + 1, dishs));
        dishs[h.d] = before;

        ans
    }
}
