use proconio::input;
fn main() {
    input! {
        _a: i64,
        b: i64,
    };

    input! {
        c: i64,
        _d: i64,
    };

    println!("{}", b - c);
}
