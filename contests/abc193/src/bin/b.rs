use std::cmp::min;
use proconio::input;
fn main() {
    input! {
        n: usize,
    }

    let mut min_p = -1;
    for _i in 0..n {
        input!{
            a: i64,
            p: i64,
            x: i64,
        }

        let stock = x - a;
        if stock > 0 {
            if min_p < 0 {
                min_p = p;
            } else {
            min_p = min(p, min_p);
            }
        }
    }
    println!("{}", min_p);
}
