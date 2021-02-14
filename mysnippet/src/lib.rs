use cargo_snippet::snippet;
use std::collections::HashMap;
use std::collections::HashSet;

mod graph;
mod matrix;
mod search;

#[snippet(name = "myfunc read_line")]
#[allow(dead_code)]
fn read_line() -> String {
    let mut line = String::new();
    std::io::stdin().read_line(&mut line).unwrap();
    line.trim_end().to_owned()
}

#[snippet(name = "myfunc is_prime")]
// 素数判定
#[allow(dead_code)]
fn is_prime(n: u64) -> bool {
    if n == 2 {
        return true;
    }
    if n == 0 || n == 1 || n % 2 == 0 {
        return false;
    }

    let mut i = 3;
    while i * i <= n {
        if n % i == 0 {
            return false;
        }

        i += 2;
    }

    true
}

#[test]
fn test_is_prime_0() {
    assert_eq!(is_prime(0), false)
}

#[test]
fn test_is_prime_1() {
    assert_eq!(is_prime(1), false)
}

#[test]
fn test_is_prime_2() {
    assert_eq!(is_prime(2), true)
}

#[test]
fn test_is_prime_3() {
    assert_eq!(is_prime(3), true)
}

#[test]
fn test_is_prime_4() {
    assert_eq!(is_prime(4), false)
}

#[test]
fn test_is_prime_5() {
    assert_eq!(is_prime(5), true)
}

#[snippet(name = "myfunc enum_divisors")]
#[snippet(prefix = "use std::collections::HashSet;")]
// 約数列挙
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

#[test]
fn test_enum_divisors_0() {
    let want: HashSet<u64> = vec![].into_iter().collect();
    assert_eq!(enum_divisors(0), want)
}

#[test]
fn test_enum_divisors_1() {
    let want: HashSet<u64> = vec![1].into_iter().collect();
    assert_eq!(enum_divisors(1), want)
}

#[test]
fn test_enum_divisors_2() {
    let want: HashSet<u64> = vec![1, 2].into_iter().collect();
    assert_eq!(enum_divisors(2), want)
}

#[test]
fn test_enum_divisors_10() {
    let want: HashSet<u64> = vec![1, 2, 5, 10].into_iter().collect();
    assert_eq!(enum_divisors(10), want)
}

#[snippet(name = "myfunc prime_factorize")]
#[snippet(prefix = "use std::collections::HashMap;")]
// 素因数分解
#[allow(dead_code)]
fn prime_factorize(n: u64) -> HashMap<u64, u64> {
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

#[test]
fn test_prime_factorize_0() {
    let want: HashMap<u64, u64> = HashMap::new();
    assert_eq!(prime_factorize(0), want);
}

#[test]
fn test_prime_factorize_1() {
    let want: HashMap<u64, u64> = HashMap::new();
    assert_eq!(prime_factorize(1), want);
}

#[test]
fn test_prime_factorize_2() {
    let mut want: HashMap<u64, u64> = HashMap::new();
    want.insert(2, 1);
    assert_eq!(prime_factorize(2), want);
}

#[test]
fn test_prime_factorize_2020() {
    let mut want: HashMap<u64, u64> = HashMap::new();
    want.insert(2, 2);
    want.insert(5, 1);
    want.insert(101, 1);
    assert_eq!(prime_factorize(2020), want);
}

#[snippet(name = "myfunc gcd")]
// 最大公約数
#[allow(dead_code)]
fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

#[snippet(name = "myfunc lcm")]
// 最小公倍数
#[allow(dead_code)]
fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}
