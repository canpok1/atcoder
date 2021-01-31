use std::collections::BTreeMap;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

struct User {
    s: i64,
    t: i64,
    p: i64,
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let w = iter.next().unwrap().parse().unwrap();

    let mut users: Vec<User> = Vec::new();
    (0..n).for_each(|_| {
        let stdin = read_line();
        let mut iter = stdin.split_whitespace();
        let s = iter.next().unwrap().parse().unwrap();
        let t = iter.next().unwrap().parse().unwrap();
        let p = iter.next().unwrap().parse().unwrap();
        users.push(User { s: s, t: t, p: p });
    });

    let solver = Solver::new(w, users);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    w: i64,
    users: Vec<User>,
}

impl Solver {
    fn new(w: i64, users: Vec<User>) -> Solver {
        Solver { w: w, users: users }
    }

    fn solve(&self) -> Vec<String> {
        let mut events: BTreeMap<i64, i64> = BTreeMap::new();
        for u in &self.users {
            let w = match events.get(&u.s) {
                Some(n) => n + u.p,
                None => u.p,
            };
            events.insert(u.s, w);

            let w = match events.get(&u.t) {
                Some(n) => n - u.p,
                None => -u.p,
            };
            events.insert(u.t, w);
        }

        let mut ans = "Yes";
        let mut required = 0;
        for (_time, e) in events.iter() {
            required += e;
            if required > self.w {
                ans = "No";
                break;
            }
        }

        let mut buf = Vec::new();
        buf.push(format!("{}", ans));
        buf
    }
}
