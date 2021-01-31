fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let a: isize = iter.next().unwrap().parse().unwrap();
    let b: isize = iter.next().unwrap().parse().unwrap();
    let c: isize = iter.next().unwrap().parse().unwrap();

    let winnter = if a == b && c == 1 {
        "Takahashi"
    } else if a == b && c == 0 {
        "Aoki"
    } else if a > b {
        "Takahashi"
    } else {
        "Aoki"
    };

    println!("{}", winnter)
}
