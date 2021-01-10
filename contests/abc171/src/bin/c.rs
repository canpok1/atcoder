use std::char::from_u32;

fn main() {
    let n: u64 = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let stdout = solve(n);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(mut n: u64) -> Vec<String> {
    let mut name = "".to_string();
    n = n - 1;

    loop {
        let char_index = (n % 26) as u32;
        name = format!("{}{}", from_u32(('a' as u32) + char_index).unwrap(), name);
        n = n / 26;

        if n == 0 {
            break;
        }
        n -= 1;
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", name));
    buf
}

#[test]
fn test_solve_1() {
    assert_eq!(solve(2), vec!("b"));
}

#[test]
fn test_solve_2() {
    assert_eq!(solve(27), vec!("aa"));
}

#[test]
fn test_solve_3() {
    assert_eq!(solve(123456789), vec!("jjddja"));
}

#[test]
fn test_solve_4() {
    assert_eq!(solve(1), vec!("a"));
}

#[test]
fn test_solve_5() {
    assert_eq!(solve(702), vec!("zz"));
}

#[test]
fn test_solve_6() {
    assert_eq!(solve(475254), vec!("zzzz"));
}
