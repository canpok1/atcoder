use std::collections::HashMap;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let solver = Solver::new(read_line());
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    s: String,
}

impl Solver {
    fn new(s: String) -> Solver {
        Solver { s: s }
    }

    fn solve(&self) -> Vec<String> {
        let mut buf = Vec::new();

        if self.s.len() == 1 {
            let ans = if self.s == "8" { "Yes" } else { "No" };
            buf.push(format!("{}", ans));
            return buf;
        }

        if self.s.len() == 2 {
            let c0 = self.s.chars().nth(0).unwrap();
            let c1 = self.s.chars().nth(1).unwrap();
            let n1: u32 = format!("{}{}", c0, c1).parse().unwrap();
            let n2: u32 = format!("{}{}", c1, c0).parse().unwrap();
            let ans = if n1 % 8 == 0 || n2 % 8 == 0 {
                "Yes"
            } else {
                "No"
            };
            buf.push(format!("{}", ans));
            return buf;
        }

        let mut char_counts: HashMap<u32, u32> = HashMap::new();
        self.s.chars().for_each(|c| {
            let key: u32 = format!("{}", c).parse().unwrap();
            char_counts.insert(
                key,
                match char_counts.get(&key) {
                    Some(count) => count + 1,
                    None => 1,
                },
            );
        });

        let mut can_make = false;
        let mut i = 112;
        while i < 10000 {
            let mut counts: HashMap<u32, u32> = HashMap::new();
            let mut num = i;
            while num != 0 {
                let key: u32 = num % 10;
                counts.insert(
                    key,
                    match counts.get(&key) {
                        Some(cnt) => cnt + 1,
                        None => 1,
                    },
                );
                num = num / 10;
            }
            can_make = counts.iter().all(|(num, cnt)| {
                if !char_counts.contains_key(&num) {
                    false
                } else if char_counts.get(&num).unwrap() < cnt {
                    false
                } else {
                    true
                }
            });
            if can_make {
                break;
            }

            i += 8;
        }

        buf.push(format!("{}", if can_make { "Yes" } else { "No" }));
        buf
    }
}

#[test]
fn test_solve_1() {
    let solver = Solver::new("1".to_owned());
    assert_eq!(solver.solve(), vec!("No"));
}

#[test]
fn test_solve_2() {
    let solver = Solver::new("8".to_owned());
    assert_eq!(solver.solve(), vec!("Yes"));
}

#[test]
fn test_solve_3() {
    let solver = Solver::new("17".to_owned());
    assert_eq!(solver.solve(), vec!("No"));
}

#[test]
fn test_solve_4() {
    let solver = Solver::new("69".to_owned());
    assert_eq!(solver.solve(), vec!("Yes"));
}

#[test]
fn test_solve_5() {
    let solver = Solver::new("131".to_owned());
    assert_eq!(solver.solve(), vec!("No"));
}

#[test]
fn test_solve_6() {
    let solver = Solver::new("823".to_owned());
    assert_eq!(solver.solve(), vec!("Yes"));
}
