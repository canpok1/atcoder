use std::collections::HashMap;

fn main() {
    let mut stdin: Vec<String> = Vec::new();
    loop {
        let mut s = String::new();
        let size = std::io::stdin().read_line(&mut s).unwrap();
        if size == 0 {
            break;
        }
        stdin.push(s.trim_end().to_string());
    }
    run(stdin).iter().for_each(|v| println!("{}", v));
}

fn run(stdin: Vec<String>) -> Vec<String> {
    let mut buf = Vec::new();

    let mut x_map: HashMap<isize, usize> = HashMap::new();
    let mut y_map: HashMap<isize, usize> = HashMap::new();
    let mut points: Vec<Point> = Vec::new();

    let mut found = false;
    for (i, line) in stdin.iter().enumerate() {
        if i == 0 {
            continue;
        }

        let (x, y) = {
            let mut ws = line.trim_end().split_whitespace();
            let n1: isize = ws.next().unwrap().parse().unwrap();
            let n2: isize = ws.next().unwrap().parse().unwrap();
            (n1, n2)
        };
        points.push(Point {
            x: x as f32,
            y: y as f32,
        });

        let count = match x_map.get(&x) {
            Some(v) => v + 1,
            None => 1,
        };
        if count >= 3 {
            found = true;
        }
        x_map.insert(x, count);

        let count = match y_map.get(&y) {
            Some(v) => v + 1,
            None => 1,
        };
        if count >= 3 {
            found = true;
        }
        y_map.insert(y, count);
    }

    for p1 in &points {
        for p2 in &points {
            if p2.x == p1.x && p2.y == p1.y {
                continue;
            }

            let slope1 = (p1.y - p2.y) / (p1.x - p2.x);
            let slope2 = (p2.y - p1.y) / (p2.x - p1.x);

            for p3 in &points {
                if p3.x == p1.x && p3.y == p1.y {
                    continue;
                }
                if p3.x == p2.x && p3.y == p2.y {
                    continue;
                }

                let slope = (p1.y - p3.y) / (p1.x - p3.x);
                if slope == slope1 || slope == slope2 {
                    found = true;
                    break;
                }
            }

            if found {
                break;
            }
        }
        if found {
            break;
        }
    }

    if found {
        buf.push(format!("Yes"));
    } else {
        buf.push(format!("No"));
    }
    buf
}

#[derive(Debug)]
struct Point {
    x: f32,
    y: f32,
}

#[test]
fn test_sample_1() {
    let input: Vec<String> = ["4", "0 1", "0 2", "0 3", "1 1"]
        .iter()
        .map(|v| v.to_string())
        .collect();
    assert_eq!(run(input), vec!("Yes"));
}

#[test]
fn test_sample_2() {
    let input: Vec<String> = [
        "14", "5 5", "0 1", "2 5", "8 0", "2 1", "0 0", "3 6", "8 6", "5 9", "7 9", "3 4", "9 2",
        "9 8", "7 2",
    ]
    .iter()
    .map(|v| v.to_string())
    .collect();

    assert_eq!(run(input), vec!("No"));
}

#[test]
fn test_sample_3() {
    let input: Vec<String> = [
        "9", "8 2", "2 3", "1 3", "3 7", "1 0", "8 8", "5 6", "9 7", "0 1",
    ]
    .iter()
    .map(|v| v.to_string())
    .collect();

    assert_eq!(run(input), vec!("Yes"));
}
