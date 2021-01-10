use std::char;
use std::collections::HashMap;

fn main() {
    let (n, _q) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n: usize = ws.next().unwrap().parse().unwrap();
        let q: usize = ws.next().unwrap().parse().unwrap();
        (n, q)
    };

    let mut balls: Vec<Ball> = (0..n).map(|i| Ball{index: i}).collect();
    let mut comparator = Comparator::new();

    let mut end = n - 1;
    while end > 0 {
        for l_index in 0..end {
            let r_index = l_index + 1;
            let l_ball = &balls[l_index];
            let r_ball = &balls[r_index];
            if comparator.compare(l_ball, r_ball) {
                balls.swap(l_index, r_index);
            }
        }
        end = end - 1;
    }

    let mut ans = String::from("! ");
    for ball in balls {
        ans.push_str(&ball.to_string());
    }
    println!("{}", ans);
}

#[derive(Debug)]
struct Ball {
    index: usize
}

impl Ball {
    fn to_string(&self) -> String {
        let ascii = (65 + self.index) as u32;
        char::from_u32(ascii).unwrap().to_string()
    }
}

struct Comparator {
    rules: HashMap<String, bool>
}

impl Comparator {
    fn new() -> Comparator {
        Comparator {
            rules: HashMap::new()
        }
    }

    fn compare(&mut self, l: &Ball, r: &Ball) -> bool {
        let key = l.to_string() + "-" + &r.to_string();

        let should_swap = match self.rules.get(&key) {
            Some(rule) => {
                *rule
            },
            None => {
                println!("? {} {}", l.to_string(), r.to_string());
                let mut line = String::new();
                std::io::stdin().read_line(&mut line).unwrap();
                let should_swap = line.trim_end() == ">";

                self.rules.insert(key, should_swap.clone());

                let key = r.to_string() + "-" + &l.to_string();
                self.rules.insert(key.clone(), !should_swap.clone());

                should_swap
            }
        };

        should_swap
    }
}