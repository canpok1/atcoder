fn main() {
    let (r1, c1) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1: isize = ws.next().unwrap().parse().unwrap();
        let n2: isize = ws.next().unwrap().parse().unwrap();
        (n1, n2)
    };
    let (r2, c2) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1: isize = ws.next().unwrap().parse().unwrap();
        let n2: isize = ws.next().unwrap().parse().unwrap();
        (n1, n2)
    };

    for o in run(r1, c1, r2, c2) {
        println!("{}", o);
    }
}

fn run(r1: isize, c1: isize, r2: isize, c2: isize) -> Vec<String> {
    let mut buf = Vec::new();

    if r1 == r2 && c1 == c2 {
        buf.push(format!("{}", 0));
        return buf;
    }

    if can_move(r1, c1, r2, c2) {
        buf.push(format!("{}", 1));
        return buf;
    }

    let x = ((r1 - c1 + r2 + c2) as f64) / 2_f64;
    let y = ((-r1 + c1 + r2 + c2) as f64) / 2_f64;

    if (x - x.floor() == 0.0) && (y - y.floor() == 0.0) {
        buf.push(format!("{}", 2));
        return buf;
    }

    if ((r1 as f64) - x).powf(2.0) + ((c1 as f64) - y).powf(2.0) <= 9_f64 {
        buf.push(format!("{}", 2));
        return buf;
    }

    if ((r2 as f64) - x).powf(2.0) + ((c2 as f64) - y).powf(2.0) <= 9_f64 {
        buf.push(format!("{}", 2));
        return buf;
    }

    buf.push(format!("{}", 3));
    buf
}

fn can_move(a: isize, b: isize, c: isize, d: isize) -> bool {
    if a + b == c + d {
        true
    } else if a - b == c - d {
        true
    } else if (a - c).abs() + (b - d).abs() <= 3 {
        true
    } else {
        false
    }
}

#[test]
fn test_run_1() {
    assert_eq!(run(1, 1, 5, 6), vec!("2"));
}

#[test]
fn test_run_2() {
    assert_eq!(run(1, 1, 1, 200001), vec!("2"));
}

#[test]
fn test_run_3() {
    assert_eq!(run(2, 3, 998244353, 998244853), vec!("3"));
}

#[test]
fn test_run_4() {
    assert_eq!(run(1, 1, 1, 1), vec!("0"));
}

#[test]
fn test_run_5() {
    assert_eq!(run(1, 1, -1, 3), vec!("1"));
}

#[test]
fn test_run_6() {
    assert_eq!(run(2, 2, 8, 8), vec!("1"));
}

#[test]
fn test_run_7() {
    assert_eq!(run(2, 2, 8, 9), vec!("2"));
}

#[test]
fn test_run_8() {
    assert_eq!(run(2, 2, -2, 6), vec!("1"));
}

#[test]
fn test_run_9() {
    assert_eq!(run(2, 2, -3, 6), vec!("2"));
}

#[test]
fn test_run_10() {
    assert_eq!(run(2, 2, -4, -4), vec!("1"));
}

#[test]
fn test_run_11() {
    assert_eq!(run(2, 2, -5, -4), vec!("2"));
}

#[test]
fn test_run_12() {
    assert_eq!(run(2, 2, 7, -3), vec!("1"));
}

#[test]
fn test_run_13() {
    assert_eq!(run(2, 2, 8, -3), vec!("2"));
}
