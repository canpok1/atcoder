fn main() {
    let (n, k) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1 = ws.next().unwrap().parse().unwrap();
        let n2 = ws.next().unwrap().parse().unwrap();
        (n1, n2)
    };
    let stdout = solve(n, k);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(n: i64, k: i64) -> Vec<String> {
    let n_min = (k - 1) as f64;
    let n_max = (n - k) as f64;
    let ans: f64 = (6.0 * n_min * n_max + 3.0 * (n_min + n_max) + 1.0) / (n.pow(3) as f64);

    let mut buf = Vec::new();
    buf.push(format!("{:.20}", ans));
    buf
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(3, 2), vec!("0.48148148148148148148"));
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(3, 1), vec!("0.25925925925925925926"));
}

#[test]
fn test_solve_3() {
    assert_eq!(solve(1000000, 1), vec!("1.00000000000000000000"));
}
