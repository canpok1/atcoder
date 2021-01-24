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
    let ans = if r1 == r2 && c1 == c2 {
        0
    } else if r1 + c1 == r2 + c2 {
        1
    } else if r1 - c1 == r2 - c2 {
        1
    } else if (r1 - r2).abs() + (c1 - c2).abs() <= 3 {
        1
    } else if (r1 + c1 + r2 + c2) % 2 == 0 {
        2
    } else if (r1 - r2).abs() + (c1 - c2).abs() <= 6 {
        2
    } else if ((r1 + c1) - (r2 + c2)).abs() <= 3 {
        2
    } else if ((r1 - c1) - (r2 - c2)).abs() <= 3 {
        2
    } else {
        3
    };

    let mut buf = Vec::new();
    buf.push(format!("{}", ans));
    buf
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

#[test]
fn test_run_14() {
    assert_eq!(run(1, 1, 1, 6), vec!("2"));
}
