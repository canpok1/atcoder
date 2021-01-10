use std::collections::HashSet;

fn main() {
    let mut stdin: Vec<String> = Vec::new();
    loop {
        let mut s = String::new();
        let size = std::io::stdin().read_line(&mut s).unwrap();
        if size == 0 {
            break;
        }
        stdin.push(s.trim_end().to_string());
    }
    run(stdin).iter().for_each(|v| println!("{}", v));
}

fn run(stdin: Vec<String>) -> Vec<String> {
    let mut buf = Vec::new();

    let (n, k) = {
        let mut ws = stdin[0].trim_end().split_whitespace();
        let n1: usize = ws.next().unwrap().parse().unwrap();
        let n2: usize = ws.next().unwrap().parse().unwrap();
        (n1, n2)
    };
    let mut cost: Vec<Vec<usize>> = Vec::new();
    for (i, v) in stdin.iter().enumerate() {
        if i == 0 {
            continue;
        }
        let t: Vec<usize> = v
            .trim_end()
            .split_whitespace()
            .map(|v| v.parse().unwrap())
            .collect();
        cost.push(t);
    }

    let mut count: usize = 0;
    let mut skip: HashSet<usize> = HashSet::new();
    skip.insert(0);
    let mut cost = 0;

    buf
}

struct Solver {
    n: usize,
}

impl Solver {
    pub fn new(n: usize) -> Solver {
        Solver { n: n }
    }

    pub fn solve(&self) -> usize {
        0
    }

    pub fn make_order_list(&self) -> Vec<Vec<usize>> {
        let ans: Vec<Vec<usize>> = Vec::new();

        // (1..self.n)..for_each(|_| {
        // })
        ans
    }
}

#[test]
fn test_sample_1() {
    assert_eq!(Solver::new(2).make_order_list(), vec![vec![1]])
}

#[test]
fn test_sample_2() {
    assert_eq!(
        Solver::new(3).make_order_list(),
        vec![vec![1, 2], vec![2, 1]]
    )
}

#[test]
fn test_sample_3() {
    assert_eq!(
        Solver::new(4).make_order_list(),
        vec![
            vec![1, 2, 3],
            vec![1, 3, 2],
            vec![2, 1, 3],
            vec![2, 3, 1],
            vec![3, 1, 2],
            vec![3, 2, 1]
        ]
    )
}
