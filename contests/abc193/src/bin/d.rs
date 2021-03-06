use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        k: i64,
        s: String,
        t: String,
    }

    let mut card_counts: Vec<i64> = vec![k; 9];
    for c in (s.clone() + &t).chars() {
        if !c.is_numeric() {
            continue;
        }
        let k = c as usize - 48;
        card_counts[k - 1] = card_counts[k - 1] - 1;
    }

    let mut win_count = 0;
    for a in 1..=9 {
        if card_counts[a - 1] <= 0 {
            continue;
        }
        let cards_a = format!("{}{}", s, a);
        let point_a = calc_point(&cards_a);

        for b in 1..=9 {
            if a == b && card_counts[b - 1] <= 1 {
                continue;
            }
            if a != b && card_counts[b - 1] <= 0 {
                continue;
            }
            let cards_b = format!("{}{}", t, b);
            let point_b = calc_point(&cards_b);

            if point_a > point_b {
                win_count += if a == b {
                    card_counts[a - 1] * (card_counts[b - 1] - 1)
                } else {
                    card_counts[a - 1] * card_counts[b - 1]
                };
            }
        }
    }
    let ans = win_count as f64 / ((9 * k - 8) * (9 * k - 9)) as f64;

    println!("{}", ans);
}

fn calc_point(cards: &str) -> i64 {
    let mut counts: HashMap<i64, u32> = HashMap::new();
    for c in cards.chars() {
        if !c.is_numeric() {
            continue;
        }
        let key = c as i64 - 48;
        let count = if counts.contains_key(&key) {
            counts[&key] + 1
        } else {
            1
        };
        counts.insert(key, count);
    }
    let mut point: i64 = 0;
    for i in 1..=9 {
        point += if counts.contains_key(&i) {
            i * 10i64.pow(counts[&i])
        } else {
            i
        };
    }
    point
}
