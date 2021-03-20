use proconio::input;
fn main() {
    input! {
        n: i64,
    };

    let keta_max = format!("{}", n).len() / 2;
    let mut ans = 0;
    for keta in 1..=keta_max {
        if keta < keta_max {
            ans += 9 * 10i64.pow((keta as u32) - 1);
            continue;
        }

        let begin = 10i64.pow((keta as u32) - 1);
        let end = 10i64.pow(keta as u32) - 1;
        for num in begin..=end {
            let num2: i64 = format!("{}{}", num, num).parse().unwrap();
            if num2 > n {
                break;
            }
            ans += 1;
        }
    }

    println!("{}", ans)
}
