use std::collections::HashMap;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let n = read_line().parse().unwrap();
    let a = read_line()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let stdout = solve(n, a);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(_n: i64, aa: Vec<i64>) -> Vec<String> {
    let mut answer = 0;
    let mut ll: HashMap<i64, i64> = HashMap::new();
    for (i, a) in aa.iter().enumerate() {
        let r = ((i + 1) as i64) - *a;
        if ll.contains_key(&r) {
            answer += ll.get(&r).unwrap();
        }

        let l = ((i + 1) as i64) + *a;
        let c = if ll.contains_key(&l) {
            ll.get(&l).unwrap() + 1
        } else {
            1
        };
        ll.insert(l, c);
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", answer));
    buf
}
