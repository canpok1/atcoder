use proconio::input;
use std::cmp::max;
fn main() {
    input! {
        x: String,
        m: i64,
    }

    let s: Vec<_> = format!("{}", x).chars().collect();

    // xx = vec![1,2,3] // 123
    let xx: Vec<i64> = s
        .iter()
        .map(|c| {
            let c = format!("{}", c);
            c.parse().unwrap()
        })
        .collect();
    let d: i64 = *xx.iter().max().unwrap();

    if xx.len() == 1 {
        let ans = if let Some(num) = f(&xx, d + 1) {
            if num <= m {
                1
            } else {
                0
            }
        } else {
            0
        };
        println!("{}", ans);
        return;
    }

    let ans = {
        let (_ng_min, ok_max) = binary_search(10i64.pow(18), 1, |n| {
            if let Some(ans) = f(&xx, n) {
                return ans <= m;
            } else {
                return false;
            }
        });
        max(0, ok_max - (d + 1) + 1)
    };
    println!("{}", ans);
}

fn f(xx: &Vec<i64>, n: i64) -> Option<i64> {
    let mut ans: i64 = 0;
    for (i, x) in xx.iter().rev().enumerate() {
        let (pow, overflow) = n.overflowing_pow(i as u32);
        if overflow {
            return None;
        }
        let (mul, overflow) = pow.overflowing_mul(*x);
        if overflow {
            return None;
        }
        ans += mul;
    }
    Some(ans)
}

#[allow(dead_code)]
fn binary_search<F>(mut ng_idx: i64, mut ok_idx: i64, is_ok: F) -> (i64, i64)
where
    F: Fn(i64) -> bool,
{
    while (ok_idx - ng_idx).abs() > 1 {
        let mid_idx = (ok_idx + ng_idx) / 2;
        if is_ok(mid_idx) {
            ok_idx = mid_idx;
        } else {
            ng_idx = mid_idx;
        }
    }
    (ng_idx, ok_idx)
}
