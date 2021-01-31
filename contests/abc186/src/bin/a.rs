fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let n: u32 = iter.next().unwrap().parse().unwrap();
    let w: u32 = iter.next().unwrap().parse().unwrap();

    println!("{}", n / w);
}
