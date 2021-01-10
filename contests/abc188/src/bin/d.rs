use std::cmp::max;
use std::collections::BTreeMap;

struct Service {
    a: i64,
    b: i64,
    c: i64,
}

fn main() {
    let (n, c) = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        let mut ws = line.trim_end().split_whitespace();
        let n1 = ws.next().unwrap().parse().unwrap();
        let n2 = ws.next().unwrap().parse().unwrap();
        (n1, n2)
    };

    let mut services = Vec::new();
    (0..n).for_each(|_| {
        let (a, b, c) = {
            let mut line = String::new();
            std::io::stdin().read_line(&mut line).unwrap();
            let mut ws = line.trim_end().split_whitespace();
            let n1 = ws.next().unwrap().parse().unwrap();
            let n2 = ws.next().unwrap().parse().unwrap();
            let n3 = ws.next().unwrap().parse().unwrap();
            (n1, n2, n3)
        };
        services.push(Service { a, b, c })
    });

    let stdout = solve(c, services);
    stdout.iter().for_each(|s| {
        println!("{}", s);
    })
}

fn solve(c: i64, ss: Vec<Service>) -> Vec<String> {
    // key-1日目とkey日目の間にvalue円の変動あり
    let mut events: BTreeMap<i64, i64> = BTreeMap::new();
    let mut max_index = 0;
    ss.iter().for_each(|s| {
        let key = s.a - 1;
        events.insert(
            key,
            if events.contains_key(&key) {
                events.get(&key).unwrap() + s.c
            } else {
                s.c
            },
        );
        let key = (s.b - 1) + 1;
        events.insert(
            key,
            if events.contains_key(&key) {
                events.get(&key).unwrap() - s.c
            } else {
                -s.c
            },
        );
        max_index = max(max_index, key);
    });

    let mut total = 0;
    let mut non_prime_price = 0;
    let mut index = 0;
    for (k, v) in events.iter() {
        total += (k - index)
            * if non_prime_price > c {
                c
            } else {
                non_prime_price
            };
        non_prime_price += v;
        index = *k;
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", total));
    buf
}
