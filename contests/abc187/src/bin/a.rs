fn main() {
    let (a, b) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1: usize = ws.next().unwrap().parse().unwrap();
        let n2: usize = ws.next().unwrap().parse().unwrap();
        (n1, n2)
    };

    let stdout = solve(a, b);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(mut a: usize, mut b: usize) -> Vec<String> {
    let mut sa = 0;
    while a > 0 {
        sa += a % 10;
        a = a / 10;
    }

    let mut sb = 0;
    while b > 0 {
        sb += b % 10;
        b = b / 10;
    }

    let mut buf = Vec::new();
    if sa >= sb {
        buf.push(format!("{}", sa));
    } else {
        buf.push(format!("{}", sb));
    }

    buf
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(123, 234), vec!("9"));
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(593, 953), vec!("17"));
}

#[test]
fn test_solve_3() {
    assert_eq!(solve(100, 999), vec!("27"));
}
