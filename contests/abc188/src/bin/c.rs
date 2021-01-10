use std::cmp::min;

fn main() {
    let _n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };
    let a = {
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

fn solve(a: Vec<u32>) -> Vec<String> {
    let len = a.len();
    let begin1: usize = 0;
    let begin2: usize = if len % 2 == 1 { len / 2 + 1 } else { len / 2 };
    let a1 = a[begin1..begin2].iter().max();
    let a2 = a[begin2..].iter().max();
    let loser_rate = min(a1, a2).unwrap();

    let mut buf = Vec::new();
    for (i, rate) in a.iter().enumerate() {
        if rate == loser_rate {
            buf.push(format!("{}", i + 1));
            break;
        }
    }

    buf
}
