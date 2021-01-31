fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let a: Vec<i64> = read_line()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();
    println!("{}", a.iter().min().unwrap());
}
