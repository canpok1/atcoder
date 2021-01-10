mod math;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let stdout = solve(&stdin);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(s: &str) -> Vec<String> {
    let mut buf = Vec::new();
    buf.push(format!("{}", s));
    buf
}

#[test]
fn test_solve_1() {
    assert_eq!(solve("1"), vec!("1"));
}
