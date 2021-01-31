fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().unwrap();
    let s: i64 = iter.next().unwrap().parse().unwrap();
    let d: i64 = iter.next().unwrap().parse().unwrap();

    let mut answer = "No";
    (0..n).for_each(|_| {
        let stdin = read_line();
        let mut iter = stdin.split_whitespace();
        let x: i64 = iter.next().unwrap().parse().unwrap();
        let y: i64 = iter.next().unwrap().parse().unwrap();

        if x >= s {
            return;
        }
        if y <= d {
            return;
        }

        answer = "Yes";
    });

    println!("{}", answer);
}
