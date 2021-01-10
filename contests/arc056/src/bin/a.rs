use std::cmp::min;

fn main() {
    let (a, b, k, l) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1 = ws.next().unwrap().parse().unwrap();
        let n2 = ws.next().unwrap().parse().unwrap();
        let n3 = ws.next().unwrap().parse().unwrap();
        let n4 = ws.next().unwrap().parse().unwrap();
        (n1, n2, n3, n4)
    };
    let stdout = solve(a, b, k, l);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(a: i64, b: i64, k: i64, l: i64) -> Vec<String> {
    let set_count = k / l;
    let mut total = min(a * set_count * l, b * set_count);
    let single_count = k - l * set_count;
    if single_count > 0 {
        total += min(a * single_count, b);
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", total));
    buf
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(3, 7, 10, 3), vec!("24"));
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(4, 5, 11, 3), vec!("20"));
}

#[test]
fn test_solve_3() {
    assert_eq!(solve(3, 8, 3, 3), vec!("8"));
}

#[test]
fn test_solve_4() {
    assert_eq!(solve(3, 8, 2, 3), vec!("6"));
}

#[test]
fn test_solve_5() {
    assert_eq!(
        solve(1000000000, 1000000000, 1000000000, 1000000000),
        vec!("1000000000")
    );
}
