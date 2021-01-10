fn main() {
    let (m, n, first_n) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1: usize = ws.next().unwrap().parse().unwrap();
        let n2: usize = ws.next().unwrap().parse().unwrap();
        let n3: usize = ws.next().unwrap().parse().unwrap();
        (n1, n2, n3)
    };

    for o in solve(m, n, first_n) {
        println!("{}", o);
    }
}

fn solve(m: usize, n: usize, first_n: usize) -> Vec<String> {
    let mut total = first_n;
    let mut stocks = first_n;
    while stocks >= m {
        let (p, s) = make_pencil(m, n, stocks);
        total += p;
        stocks = p + s;
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", total));
    buf
}

fn make_pencil(m: usize, n: usize, stocks: usize) -> (usize, usize) {
    let producsts = stocks / m * n;
    let new_stocks = stocks % m;
    (producsts, new_stocks)
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(2, 1, 8), vec!("15"));
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(7, 4, 30), vec!("62"));
}

#[test]
fn test_solve_3() {
    assert_eq!(solve(100, 99, 1000), vec!("90199"));
}

#[test]
fn test_solve_4() {
    assert_eq!(solve(2, 1, 1), vec!("1"));
}

#[test]
fn test_solve_5() {
    assert_eq!(solve(2, 1, 2), vec!("3"));
}
