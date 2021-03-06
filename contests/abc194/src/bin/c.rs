use proconio::input;
fn main() {
    input! {
        n: usize,
        aa: [i64; n],
    };

    let mut sums1: Vec<i64> = vec![];
    let mut sums2: Vec<i64> = vec![];
    let mut sum1 = 0;
    let mut sum2 = 0;
    for a in aa.iter() {
        sum1 += a;
        sum2 += a * a;
        sums1.push(sum1);
        sums2.push(sum2);
    }

    let mut ans = 0;
    for i in 2..=n {
        let ai = aa[i - 1];
        let aj1 = sums1[i - 2];
        let aj2 = sums2[i - 2];
        ans += (i - 1) as i64 * ai * ai - 2 * ai * aj1 + aj2;
    }

    println!("{}", ans);
}
