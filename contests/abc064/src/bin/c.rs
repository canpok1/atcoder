use std::collections::HashSet;

fn main() {
    let _n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let a: Vec<usize> = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect()
    };

    let stdout = solve(a);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(aa: Vec<usize>) -> Vec<String> {
    let mut colors: HashSet<String> = HashSet::new();
    let mut other_count = 0;
    for a in aa {
        match a {
            0..=399 => {
                colors.insert("灰色".to_string());
            }
            400..=799 => {
                colors.insert("茶色".to_string());
            }
            800..=1199 => {
                colors.insert("緑色".to_string());
            }
            1200..=1599 => {
                colors.insert("水色".to_string());
            }
            1600..=1999 => {
                colors.insert("青色".to_string());
            }
            2000..=2399 => {
                colors.insert("黃色".to_string());
            }
            2400..=2799 => {
                colors.insert("橙色".to_string());
            }
            2800..=3199 => {
                colors.insert("赤色".to_string());
            }
            _ => {
                other_count += 1;
            }
        }
    }
    let color_count = colors.len();
    let min = if color_count == 0 && other_count == 0 {
        0
    } else if color_count == 0 && other_count > 0 {
        1
    } else {
        color_count
    };
    let max = color_count + other_count;

    let mut buf = Vec::new();
    buf.push(format!("{} {}", min, max));
    buf
}

#[test]
fn test_solve_1() {
    let a: Vec<usize> = vec![2100, 2500, 2700, 2700];
    assert_eq!(solve(a), vec!("2 2"));
}

#[test]
fn test_solve_2() {
    let a = vec![1100, 1900, 2800, 3200, 3200];
    assert_eq!(solve(a), vec!("3 5"));
}

#[test]
fn test_solve_3() {
    let a = vec![
        800, 810, 820, 830, 840, 850, 860, 870, 880, 890, 900, 910, 920, 930, 940, 950, 960, 970,
        980, 990,
    ];
    assert_eq!(solve(a), vec!("1 1"));
}

#[test]
fn test_solve_4() {
    let a = vec![3200, 3200];
    assert_eq!(solve(a), vec!("1 2"));
}

#[test]
fn test_solve_5() {
    let a = vec![1, 3200, 3200];
    assert_eq!(solve(a), vec!("1 3"));
}
