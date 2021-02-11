#[allow(dead_code)]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let v: i64 = iter.next().unwrap().parse().unwrap();
    let t: i64 = iter.next().unwrap().parse().unwrap();
    let s: i64 = iter.next().unwrap().parse().unwrap();
    let d: i64 = iter.next().unwrap().parse().unwrap();

    let start = v * t;
    let end = v * s;

    if start <= d && d <= end {
        println!("No");
    } else {
        println!("Yes");
    }
}
