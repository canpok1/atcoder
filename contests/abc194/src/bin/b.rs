use proconio::input;
use std::cmp::max;
use std::cmp::min;

struct Worker {
    a: i64,
    b: i64,
}

fn main() {
    input! {
        n: usize,
    };
    let mut workers: Vec<Worker> = vec![];
    for _ in 0..n {
        input! {
            a: i64,
            b: i64,
        };
        workers.push(Worker { a: a, b: b });
    }

    let mut ans = std::i64::MAX;
    for a in 0..n {
        for b in 0..n {
            ans = if a == b {
                min(ans, workers[a].a + workers[b].b)
            } else {
                min(ans, max(workers[a].a, workers[b].b))
            };
        }
    }

    println!("{}", ans);
}
