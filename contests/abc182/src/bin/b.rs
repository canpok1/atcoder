use std::collections::HashMap;
use std::collections::HashSet;

#[allow(dead_code)]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

#[allow(dead_code)]
fn enum_divisors(n: u64) -> HashSet<u64> {
    let mut divisors = HashSet::new();
    if n == 0 {
        return divisors;
    }
    let mut i = 1;
    while i * i <= n {
        if n % i == 0 {
            divisors.insert(i);
            divisors.insert(n / i);
        }
        i += 1;
    }
    divisors
}

fn main() {
    let _n: usize = read_line().parse().unwrap();
    let aa: Vec<u64> = read_line()
        .split_whitespace()
        .map(|v| v.parse().unwrap())
        .collect();

    let mut divisors_count: HashMap<u64, u64> = HashMap::new();
    let mut max_d = 0;
    let mut max_count = 0;
    for a in aa {
        let divisors = enum_divisors(a);
        for d in divisors {
            if d == 1 {
                continue;
            }
            let count = if divisors_count.contains_key(&d) {
                divisors_count.get(&d).unwrap() + 1
            } else {
                1
            };
            if count >= max_count {
                max_d = d;
                max_count = count;
            }
            divisors_count.insert(d, count);
        }
    }

    println!("{}", max_d);
}
