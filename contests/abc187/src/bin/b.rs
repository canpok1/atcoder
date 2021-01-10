struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let mut points: Vec<Point> = Vec::new();
    (0..n).for_each(|_| {
        let (x, y) = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut ws = line.trim_end().split_whitespace();
            let n1: isize = ws.next().unwrap().parse().unwrap();
            let n2: isize = ws.next().unwrap().parse().unwrap();
            (n1, n2)
        };

        points.push(Point {
            x: x as f64,
            y: y as f64,
        });
    });

    let stdout = solve(points);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(points: Vec<Point>) -> Vec<String> {
    let mut count = 0;
    for (i, p1) in points.iter().enumerate() {
        for (j, p2) in points.iter().enumerate() {
            if i >= j {
                continue;
            }

            let a = (p2.y - p1.y) / (p2.x - p1.x);
            if a >= -1.0 && a <= 1.0 {
                count += 1;
            }
        }
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", count));
    buf
}

#[test]
fn test_solve_1() {
    let points: Vec<Point> = vec![
        Point { x: 0.0, y: 0.0 },
        Point { x: 1.0, y: 2.0 },
        Point { x: 2.0, y: 1.0 },
    ];
    assert_eq!(solve(points), vec!("2"));
}

#[test]
fn test_solve_2() {
    let points: Vec<Point> = vec![Point {
        x: -691.0,
        y: 273.0,
    }];
    assert_eq!(solve(points), vec!("0"));
}
