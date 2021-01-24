use std::collections::BTreeSet;

fn main() {
    let mut stdin: Vec<String> = Vec::new();
    loop {
        let mut s = String::new();
        let size = std::io::stdin().read_line(&mut s).unwrap();
        if size == 0 {
            break;
        }
        stdin.push(s.trim_end().to_string());
    }
    run(stdin).iter().for_each(|v| println!("{}", v));
}

fn run(stdin: Vec<String>) -> Vec<String> {
    let mut buf = Vec::new();

    let n: u64 = {
        let mut ws = stdin[0].trim_end().split_whitespace();
        ws.next().unwrap().parse().unwrap()
    };

    let ans = enum_divisors(n);
    ans.iter().for_each(|v| buf.push(format!("{}", v)));
    buf
}

fn enum_divisors(n: u64) -> BTreeSet<u64> {
    let mut divisors = BTreeSet::new();
    if n == 0 {
        return divisors;
    }

    let mut i: u64 = 1;
    while i * i <= n {
        if n % i == 0 {
            divisors.insert(i);
            divisors.insert(n / i);
        }

        i += 1;
    }

    divisors
}
