use std::cmp::max;

fn main() {
    let s: String = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().to_owned()
    };

    let t = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let stdout = solve(&s, t);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(s: &str, t: i64) -> Vec<String> {
    let mut x: i64 = 0;
    let mut y: i64 = 0;
    let mut unknown_count: i64 = 0;

    for command in s.chars() {
        match command {
            'L' => {
                x -= 1;
            }
            'R' => {
                x += 1;
            }
            'U' => {
                y += 1;
            }
            'D' => {
                y -= 1;
            }
            _ => {
                unknown_count += 1;
            }
        };
    }
    let ans = if t == 1 {
        x.abs() + y.abs() + unknown_count
    } else {
        max((s.len() % 2) as i64, x.abs() + y.abs() - unknown_count)
    };

    let mut buf = Vec::new();
    buf.push(format!("{}", ans));
    buf
}

#[test]
fn test_solve_1() {
    assert_eq!(solve("UL?", 1), vec!("3"));
}

#[test]
fn test_solve_2() {
    assert_eq!(solve("UD?", 1), vec!("1"));
}

#[test]
fn test_solve_3() {
    assert_eq!(solve("UUUU?DDR?LLLL", 1), vec!("7"));
}

#[test]
fn test_solve_4() {
    assert_eq!(solve("UULL?", 2), vec!("3"));
}
