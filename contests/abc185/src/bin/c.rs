use std::collections::HashMap;

fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

fn prime_factorize(n: u32) -> HashMap<u32, u32> {
    let mut ans = HashMap::new();
    let mut i = 2;
    let mut nn = n;
    while i * i <= nn {
        if nn % i == 0 {
            let mut ex = 0;
            while nn % i == 0 {
                ex += 1;
                nn = nn / i;
            }
            ans.insert(i, ex);
        }
        i += 1;
    }
    if nn > 1 {
        ans.insert(nn, 1);
    }

    ans
}

fn main() {
    let l: u32 = read_line().parse().unwrap();

    let mut all_primes: HashMap<u32, i32> = HashMap::new();

    for i in 0_u32..11_u32 {
        // 分子
        let primes = prime_factorize(l - 1 - i);
        for (p, cnt) in primes.iter() {
            if all_primes.contains_key(&p) {
                all_primes.insert(*p, all_primes.get(p).unwrap() + (*cnt as i32));
            } else {
                all_primes.insert(*p, *cnt as i32);
            }
        }

        // 分母
        let primes = prime_factorize(i + 1);
        for (p, cnt) in primes.iter() {
            if all_primes.contains_key(&p) {
                all_primes.insert(*p, all_primes.get(p).unwrap() - (*cnt as i32));
            } else {
                all_primes.insert(*p, (-1) * (*cnt as i32));
            }
        }
    }
    let mut ans: u64 = 1;
    for (p, cnt) in all_primes.iter() {
        if *cnt > 0 {
            ans *= (*p as u64).pow(*cnt as u32);
        } else {
            ans /= (*p as u64).pow(*cnt as u32);
        };
    }

    println!("{:.0}", ans);
}
