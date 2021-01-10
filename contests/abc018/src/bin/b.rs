fn main() {
    let s: String = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().to_owned()
    };

    let n: usize = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let mut os: Vec<Operation> = Vec::new();
    (0..n).for_each(|_| {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1: usize = ws.next().unwrap().parse().unwrap();
        let n2: usize = ws.next().unwrap().parse().unwrap();
        os.push(Operation { l: n1, r: n2 });
    });

    let stdout = solve(&s, n, os);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

struct Operation {
    l: usize,
    r: usize,
}

fn solve(s: &str, _n: usize, os: Vec<Operation>) -> Vec<String> {
    let mut ans = s.to_string();
    for o in os {
        let index_l = o.l - 1;
        let index_r = o.r - 1;
        let head: &str = &ans[..index_l];
        let rev: String = ans[index_l..(index_r + 1)].chars().rev().collect();
        let tail: &str = &ans[(index_r + 1)..];
        ans = format!("{}{}{}", head, rev, tail);
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", ans));
    buf
}

#[test]
fn test_solve_1() {
    let os = vec![Operation { l: 3, r: 5 }, Operation { l: 1, r: 4 }];
    assert_eq!(solve("abcdef", 2, os), vec!("debacf"));
}

#[test]
fn test_solve_2() {
    let os = vec![
        Operation { l: 1, r: 7 },
        Operation { l: 1, r: 2 },
        Operation { l: 3, r: 4 },
    ];
    assert_eq!(solve("redcoat", 3, os), vec!("atcoder"));
}

#[test]
fn test_solve_3() {
    let os = vec![Operation { l: 1, r: 2 }];
    assert_eq!(solve("abc", 1, os), vec!("bac"));
}
