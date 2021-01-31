fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let d: u32 = iter.next().unwrap().parse().unwrap();
    let n: u64 = iter.next().unwrap().parse().unwrap();

    let ans = if n == 100 {
        (n + 1) * 100u64.pow(d)
    } else {
        n * 100u64.pow(d)
    };
    println!("{}", ans);
}
