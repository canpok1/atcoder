struct Arm {
    x: i32,
    l: i32,
}
impl Arm {
    fn begin(&self) -> i32 {
        self.x - self.l
    }

    fn end(&self) -> i32 {
        self.x + self.l
    }
}

fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let mut arms = Vec::new();
    (0..n).for_each(|_| {
        let (x, l) = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut ws = line.trim_end().split_whitespace();
            let n1 = ws.next().unwrap().parse().unwrap();
            let n2 = ws.next().unwrap().parse().unwrap();
            (n1, n2)
        };
        arms.push(Arm { x: x, l: l });
    });

    let stdout = solve(arms);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(mut arms: Vec<Arm>) -> Vec<String> {
    arms.sort_by(|a, b| a.end().cmp(&b.end()));

    let mut count = 0;
    let mut x = -10_i32.pow(9);
    for arm in arms {
        if arm.begin() >= x {
            count += 1;
            x = arm.end();
        }
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", count));
    buf
}

#[test]
fn test_solve_1() {
    let arms = vec![
        Arm { x: 2, l: 4 },
        Arm { x: 4, l: 3 },
        Arm { x: 9, l: 3 },
        Arm { x: 100, l: 5 },
    ];
    assert_eq!(solve(arms), vec!("3"));
}

#[test]
fn test_solve_2() {
    let arms = vec![Arm { x: 8, l: 20 }, Arm { x: 1, l: 20 }];
    assert_eq!(solve(arms), vec!("1"));
}

#[test]
fn test_solve_3() {
    let arms = vec![
        Arm { x: 10, l: 1 },
        Arm { x: 2, l: 1 },
        Arm { x: 4, l: 1 },
        Arm { x: 6, l: 1 },
        Arm { x: 8, l: 1 },
    ];
    assert_eq!(solve(arms), vec!("5"));
}

#[test]
fn test_solve_10() {
    let arms = vec![
        Arm { x: 0, l: 3 },
        Arm { x: 1, l: 5 },
        Arm { x: 2, l: 1 },
        Arm { x: 3, l: 1 },
        Arm { x: 4, l: 2 },
        Arm { x: 5, l: 1 },
    ];
    assert_eq!(solve(arms), vec!("2"));
}
