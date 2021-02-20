use proconio::input;
fn main() {
    input! {
        n: i64,
        k: i64,
    }

    let mut a = n;
    for _ in 1..=k {
        a = f(a);
    }

    println!("{}", a);
}

fn f(x: i64) -> i64 {
    g1(x) - g2(x)
}

fn g1(x: i64) -> i64 {
    let mut s: Vec<_> = format!("{}", x).chars().collect();
    s.sort();
    let s: String = s.into_iter().rev().collect();
    s.parse().unwrap()
}

fn g2(x: i64) -> i64 {
    let mut s: Vec<_> = format!("{}", x).chars().collect();
    s.sort();
    let s: String = s.into_iter().collect();
    s.parse().unwrap()
}
