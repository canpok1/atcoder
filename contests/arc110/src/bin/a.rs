use std::collections::HashMap;

fn main() {
    let n: u32 = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };
    for o in run(n) {
        println!("{}", o);
    }
}

fn run(n: u32) -> Vec<String> {
    let prime_numbers: Vec<u32> = vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29];

    let mut pows: HashMap<u32, u32> = HashMap::new();
    for n in &prime_numbers {
        pows.insert(*n, 0);
    }

    for begin_num in 1..=n {
        let mut num = begin_num as u32;
        for prime_number in &prime_numbers {
            let mut count = 0;

            loop {
                if num % prime_number != 0 {
                    break;
                }

                num = num / prime_number;
                count += 1;
            }

            if count > pows[&prime_number] {
                pows.insert(*prime_number, count);
            }
        }
    }

    let mut ans: u64 = 1;
    for (k, v) in &pows {
        ans = ans * (k.pow(*v) as u64);
    }

    let mut buf = Vec::new();
    buf.push(format!("{}", ans + 1));
    buf
}

#[test]
fn test_run_1() {
    assert_eq!(run(3), vec!("7"));
}

#[test]
fn test_run_2() {
    assert_eq!(run(10), vec!("2521"));
}

#[test]
fn test_run_3() {
    assert_eq!(run(2), vec!("3"));
}

#[test]
fn test_run_4() {
    assert_eq!(run(3), vec!("7"));
}

#[test]
fn test_run_5() {
    assert_eq!(run(30), vec!("7"));
}
