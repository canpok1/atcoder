use std::cmp::min;

fn main() {
    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let s: String = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().to_owned()
    };

    let t: String = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().to_owned()
    };

    let stdout = solve(n, &s, &t);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(n: usize, s: &str, t: &str) -> Vec<String> {
    let mut match_size = min(t.len(), n);
    loop {
        if match_size == 0 || s[s.len() - match_size..] == t[..match_size] {
            break;
        }
        match_size -= 1;
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", 2 * n - match_size));
    buf
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(3, "abc", "cde"), vec!("5"));
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(1, "a", "z"), vec!("2"));
}

#[test]
fn test_solve_3() {
    assert_eq!(solve(4, "expr", "expr"), vec!("4"));
}

#[test]
fn test_solve_4() {
    assert_eq!(solve(4, "aaba", "abcd"), vec!("7"));
}

#[test]
fn test_solve_5() {
    assert_eq!(solve(4, "aaba", "abad"), vec!("5"));
}
