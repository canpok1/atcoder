struct Point {
    a: u32,
    b: u32,
}
impl Point {
    fn diff(&self) -> u32 {
        2 * self.a + self.b
    }
}

fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let mut points: Vec<Point> = Vec::new();
    (0..n).for_each(|_| {
        let (a, b) = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut ws = line.trim_end().split_whitespace();
            let n1: u32 = ws.next().unwrap().parse().unwrap();
            let n2: u32 = ws.next().unwrap().parse().unwrap();
            (n1, n2)
        };
        points.push(Point { a: a, b: b });
    });

    let stdout = solve(points);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(mut points: Vec<Point>) -> Vec<String> {
    let mut max_a: i64 = 0;
    for p in &points {
        max_a += p.a as i64;
    }
    points.sort_by(|v1, v2| v1.diff().cmp(&v2.diff()));

    let mut count = 0;
    let mut x: i64 = -max_a;
    for p in points.iter().rev() {
        x += p.diff() as i64;
        count += 1;
        if x > 0 {
            break;
        }
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", count));
    buf
}

#[test]
fn test_solve_1() {
    let points: Vec<Point> = vec![
        Point { a: 2, b: 1 },
        Point { a: 2, b: 2 },
        Point { a: 5, b: 1 },
        Point { a: 1, b: 3 },
    ];
    assert_eq!(solve(points), vec!("1"));
}

#[test]
fn test_solve_2() {
    let points: Vec<Point> = vec![
        Point { a: 2, b: 1 },
        Point { a: 2, b: 1 },
        Point { a: 2, b: 1 },
        Point { a: 2, b: 1 },
        Point { a: 2, b: 1 },
    ];
    assert_eq!(solve(points), vec!("3"));
}

#[test]
fn test_solve_3() {
    let points: Vec<Point> = vec![Point { a: 1, b: 1 }];
    assert_eq!(solve(points), vec!("1"));
}

#[test]
fn test_solve_4() {
    let points: Vec<Point> = vec![Point {
        a: 1000000000,
        b: 1000000000,
    }];
    assert_eq!(solve(points), vec!("1"));
}

#[test]
fn test_solve_5() {
    let points: Vec<Point> = vec![Point {
        a: 1,
        b: 1000000000,
    }];
    assert_eq!(solve(points), vec!("1"));
}

#[test]
fn test_solve_6() {
    let points: Vec<Point> = vec![
        Point {
            a: 1000000000,
            b: 1,
        },
        Point {
            a: 1000000000,
            b: 1,
        },
        Point {
            a: 1000000000,
            b: 1,
        },
        Point {
            a: 1000000000,
            b: 1,
        },
        Point {
            a: 1000000000,
            b: 1,
        },
    ];
    assert_eq!(solve(points), vec!("5"));
}
