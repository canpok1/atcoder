use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut ans = "Yes";

    let mut i = 1;
    for c in s.chars() {
        if i % 2 == 1 && c.is_uppercase() {
            ans = "No"
        }
        if i % 2 == 0 && c.is_lowercase() {
            ans = "No"
        }
        i += 1;
    }

    println!("{}", ans);
}
