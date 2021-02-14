use std::cmp::max;

#[allow(dead_code)]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn main() {
    let _n: usize = read_line().parse().unwrap();
    let aa: Vec<i64> = read_line()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let mut x: i64 = 0;
    let mut x_max: i64 = 0;
    let mut dx: i64 = 0;
    let mut dx_max: i64 = 0;

    for a in aa {
        dx += a;
        dx_max = max(dx_max, dx);
        x_max = max(x_max, x + dx_max);
        x += dx;
    }

    println!("{}", x_max);
}
