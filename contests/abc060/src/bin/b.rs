fn main() {
    let (a, b, c) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1: u64 = ws.next().unwrap().parse().unwrap();
        let n2: u64 = ws.next().unwrap().parse().unwrap();
        let n3: u64 = ws.next().unwrap().parse().unwrap();
        (n1, n2, n3)
    };

    let stdout = solve(a, b, c);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(a: u64, b: u64, c: u64) -> Vec<String> {
    let ans = c % gcd(a, b) == 0;

    let mut buf = Vec::new();
    buf.push(format!("{}", if ans { "YES" } else { "NO" }));
    buf
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(7, 5, 1), vec!("YES"));
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(2, 2, 1), vec!("NO"));
}

#[test]
fn test_solve_3() {
    assert_eq!(solve(1, 100, 97), vec!("YES"));
}

#[test]
fn test_solve_4() {
    assert_eq!(solve(40, 98, 58), vec!("YES"));
}

#[test]
fn test_solve_5() {
    assert_eq!(solve(77, 42, 36), vec!("NO"));
}
