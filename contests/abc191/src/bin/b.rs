#[allow(dead_code)]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let _n: usize = iter.next().unwrap().parse().unwrap();
    let x: i64 = iter.next().unwrap().parse().unwrap();
    let aa: Vec<i64> = read_line()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let mut ans: Vec<String> = Vec::new();
    for a in aa {
        if a != x {
            ans.push(format!("{}", a));
        }
    }
    println!("{}", ans.join(" "));
}
