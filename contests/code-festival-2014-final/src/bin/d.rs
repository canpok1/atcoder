fn main() {
    let a = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let stdout = solve(a);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(a: u32) -> Vec<String> {
    let mut buf = Vec::new();
    buf.push(format!("{} {}", a + 1, 2));
    buf
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(1), vec!("2 2"));
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(100), vec!("101 2"));
}

#[test]
fn test_solve_3() {
    assert_eq!(solve(1000000000), vec!("1000000001 2"));
}
