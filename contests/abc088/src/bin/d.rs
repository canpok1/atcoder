use std::collections::HashSet;
use std::collections::VecDeque;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let h = iter.next().unwrap().parse().unwrap();
    let w = iter.next().unwrap().parse().unwrap();

    let mut ss: Vec<String> = Vec::new();
    (0..h).for_each(|_| {
        let s: String = read_line().parse().unwrap();
        ss.push(s);
    });

    let mut solver = Solver::new(h, w, ss);
    let stdout = solver.solve();
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Solver {
    h: usize,
    w: usize,
    ss: Vec<String>,

    // dp[y][x]
    dp: Vec<Vec<usize>>,
}

impl Solver {
    fn new(h: usize, w: usize, ss: Vec<String>) -> Solver {
        let dp = vec![vec![std::usize::MAX; w]; h];
        Solver {
            h: h,
            w: w,
            ss: ss,
            dp: dp,
        }
    }

    fn solve(&mut self) -> Vec<String> {
        let mut visited_points: HashSet<(usize, usize)> = HashSet::new();
        let mut queue: VecDeque<(usize, usize, usize)> = VecDeque::new();
        queue.push_back((0, 0, 1));
        while queue.len() > 0 {
            let (x, y, step) = queue.pop_front().unwrap();
            if self.ss[y].chars().nth(x).unwrap() == '#' {
                continue;
            }
            if visited_points.contains(&(x, y)) {
                continue;
            }

            self.dp[y][x] = step;
            visited_points.insert((x, y));

            if x == (self.w - 1) && y == (self.h - 1) {
                continue;
            }

            // 上
            if y > 0 {
                queue.push_back((x, y - 1, step + 1));
            }
            // 下
            if y < self.h - 1 {
                queue.push_back((x, y + 1, step + 1));
            }
            // 左
            if x > 0 {
                queue.push_back((x - 1, y, step + 1));
            }
            // 右
            if x < self.w - 1 {
                queue.push_back((x + 1, y, step + 1));
            }
        }

        let mut buf = Vec::new();
        let goal_step = self.dp[self.h - 1][self.w - 1];
        if goal_step == std::usize::MAX {
            buf.push(format!("{}", -1));
        } else {
            let mut black_count = 0;
            self.ss.iter().for_each(|s| {
                black_count += s
                    .chars()
                    .fold(0, |count, c| if c == '#' { count + 1 } else { count });
            });
            buf.push(format!("{}", self.h * self.w - goal_step - black_count));
        }
        buf
    }
}
