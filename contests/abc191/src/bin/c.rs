#[allow(dead_code)]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let h: usize = iter.next().unwrap().parse().unwrap();
    let w: usize = iter.next().unwrap().parse().unwrap();

    let mut ss: Vec<String> = Vec::new();
    for _ in 0..h {
        ss.push(read_line());
    }

    let solver = Solver::new(h, w, ss);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    h: usize,
    w: usize,
    ss: Vec<String>,
}

impl Solver {
    fn new(h: usize, w: usize, ss: Vec<String>) -> Solver {
        Solver { h: h, w: w, ss: ss }
    }

    fn solve(&self) -> Vec<String> {
        let mut ans = 0;
        for y in 1..self.h {
            for x in 1..self.w {
                let mut cnt = 0;
                // 左上
                if self.ss[y - 1].chars().nth(x - 1).unwrap() == '#' {
                    cnt += 1;
                }
                // 右上
                if self.ss[y - 1].chars().nth(x).unwrap() == '#' {
                    cnt += 1;
                }
                // 左下
                if self.ss[y].chars().nth(x - 1).unwrap() == '#' {
                    cnt += 1;
                }
                // 右下
                if self.ss[y].chars().nth(x).unwrap() == '#' {
                    cnt += 1;
                }

                if cnt == 1 || cnt == 3 {
                    ans += 1;
                }
            }
        }

        let mut buf = Vec::new();
        buf.push(format!("{}", ans));
        buf
    }
}

#[test]
fn test_1() {
    let h = 5;
    let w = 5;
    let ss: Vec<String> = vec![".....", ".###.", ".###.", ".###.", "....."]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let solver = Solver::new(h, w, ss);
    let stdout = solver.solve();
    assert_eq!(stdout[0], "4");
}

#[test]
fn test_2() {
    let h = 3;
    let w = 3;
    let ss: Vec<String> = vec!["...", ".#.", "..."]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let solver = Solver::new(h, w, ss);
    let stdout = solver.solve();
    assert_eq!(stdout[0], "4");
}

#[test]
fn test_3() {
    let h = 5;
    let w = 5;
    let ss: Vec<String> = vec![".....", "...#.", "..##.", ".###.", "....."]
        .into_iter()
        .map(|s| s.to_owned())
        .collect();
    let solver = Solver::new(h, w, ss);
    let stdout = solver.solve();
    assert_eq!(stdout[0], "8");
}
