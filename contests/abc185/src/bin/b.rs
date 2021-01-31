use std::cmp::min;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let stdin = read_line();
    let mut iter = stdin.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().unwrap();
    let m: i64 = iter.next().unwrap().parse().unwrap();
    let t: i64 = iter.next().unwrap().parse().unwrap();

    let mut ans = "Yes";
    let mut battery = n;
    let mut time = 0;
    for _ in 0..m {
        let stdin = read_line();
        let mut iter = stdin.split_whitespace();
        let a: i64 = iter.next().unwrap().parse().unwrap();
        let b: i64 = iter.next().unwrap().parse().unwrap();

        battery = battery - (a - time);
        if battery <= 0 {
            ans = "No";
            break;
        }
        battery = min(n, battery + (b - a));
        time = b;
    }
    battery = battery - (t - time);
    if battery <= 0 {
        ans = "No";
    }

    println!("{}", ans);
}
