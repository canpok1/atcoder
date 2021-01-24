fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

struct Sake {
    v: u64,
    p: u64,
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let n = iter.next().unwrap().parse().unwrap();
    let x = iter.next().unwrap().parse().unwrap();

    let mut ss: Vec<Sake> = Vec::new();
    (0..n).for_each(|_| {
        let stdin = read_line();
        let mut iter = stdin.split_whitespace();
        let v = iter.next().unwrap().parse().unwrap();
        let p = iter.next().unwrap().parse().unwrap();
        ss.push(Sake { v: v, p: p });
    });

    let solver = Solver::new(x, ss);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    x: u64,
    ss: Vec<Sake>,
}

impl Solver {
    fn new(x: u64, ss: Vec<Sake>) -> Solver {
        Solver { x: x, ss: ss }
    }

    fn solve(&self) -> Vec<String> {
        let mut ans: i64 = -1;
        let mut total = 0;
        for (i, sake) in self.ss.iter().enumerate() {
            total += sake.v * sake.p;

            if total > (self.x * 100) {
                ans = (i as i64) + 1;
                break;
            }
        }

        let mut buf = Vec::new();
        buf.push(format!("{}", ans));
        buf
    }
}

#[test]
fn test_solve_1() {
    let ss = vec![Sake { v: 1, p: 0 }];
    let solver = Solver::new(0, ss);
    assert_eq!(solver.solve(), vec!("-1"));
}

#[test]
fn test_solve_2() {
    let ss = vec![Sake { v: 1, p: 1 }];
    let solver = Solver::new(0, ss);
    assert_eq!(solver.solve(), vec!("1"));
}

#[test]
fn test_solve_3() {
    let ss = vec![Sake { v: 1000, p: 100 }];
    let solver = Solver::new(1000000, ss);
    assert_eq!(solver.solve(), vec!("-1"));
}

#[test]
fn test_solve_4() {
    let ss = vec![Sake { v: 200, p: 5 }, Sake { v: 370, p: 3 }];
    let solver = Solver::new(21, ss);
    assert_eq!(solver.solve(), vec!("2"));
}

#[test]
fn test_solve_5() {
    let mut ss = Vec::new();
    (0..1000).for_each(|_| {
        ss.push(Sake { v: 1000, p: 0 });
    });
    let solver = Solver::new(0, ss);
    assert_eq!(solver.solve(), vec!("-1"));
}

#[test]
fn test_solve_6() {
    let mut ss = Vec::new();
    (0..1000).for_each(|_| {
        ss.push(Sake { v: 1000, p: 100 });
    });
    let solver = Solver::new(1000000, ss);
    assert_eq!(solver.solve(), vec!("-1"));
}

#[test]
fn test_solve_7() {
    let mut ss = Vec::new();
    (0..1000).for_each(|_| {
        ss.push(Sake { v: 1000, p: 100 });
    });
    let solver = Solver::new(999999, ss);
    assert_eq!(solver.solve(), vec!("1000"));
}

#[test]
fn test_solve_8() {
    let mut ss = Vec::new();
    (0..1000).for_each(|_| {
        ss.push(Sake { v: 1, p: 1 });
    });
    let solver = Solver::new(1, ss);
    assert_eq!(solver.solve(), vec!("101"));
}
