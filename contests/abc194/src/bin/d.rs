use proconio::input;
fn main() {
    input! {
        n: i32,
    }

    let mut ans: f64 = 0.0;
    for a in 1..=(n - 1) {
        ans += (n as f64) / (a as f64);
    }

    println!("{}", ans);
}
