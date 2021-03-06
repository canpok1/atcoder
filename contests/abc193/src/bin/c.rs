use std::collections::HashSet;
use proconio::input;
fn main() {
    input! {
        n: i64
    }

    println!("{}", f(n));
}

fn f(n: i64) -> i64 {
    let mut values : HashSet<i64> = HashSet::new();
    let mut a = 2;
    while a * a <= n {
        let mut v = a * a;
        while v <= n {
            values.insert(v);
            v *= a
        }
        a += 1;
    }
    n - (values.len() as i64)
}