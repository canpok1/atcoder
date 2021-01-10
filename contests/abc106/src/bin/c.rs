fn main() {
    let s: String = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().to_owned()
    };
    let n: u64 = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let stdout = solve(&s, n);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(s: &str, k: u64) -> Vec<String> {
    let mut ans = "".to_string();
    for (i, c) in s.chars().enumerate() {
        ans = format!("{}", c);
        if c != '1' || (i as u64) >= (k - 1) {
            break;
        }
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", ans));
    buf
}

#[test]
fn test_solve_1() {
    assert_eq!(solve("1214", 4), vec!("2"));
}

#[test]
fn test_solve_2() {
    assert_eq!(solve("3", 157), vec!("3"));
}

#[test]
fn test_solve_3() {
    assert_eq!(solve("299792458", 9460730472580800), vec!("2"));
}

#[test]
fn test_solve_4() {
    assert_eq!(solve("1234", 1), vec!("1"));
}

#[test]
fn test_solve_5() {
    assert_eq!(solve("1234", 2), vec!("2"));
}

#[test]
fn test_solve_6() {
    assert_eq!(solve("98", 1), vec!("9"));
}
