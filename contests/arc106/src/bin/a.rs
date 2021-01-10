fn main() {
    let n: u64 = {
        let mut line = String::new();
        std::io::stdin().read_line(&mut line).unwrap();
        line.trim_end().parse().unwrap()
    };

    let output = run(n);
    for o in output {
        println!("{}", o);
    }
}

fn run(n: u64) -> Vec<String> {
    let mut buf = Vec::new();

    for a in 1..=37 {
        for b in 1..=25 {
            let ans = 3u64.pow(a) + 5u64.pow(b);
            if ans == n {
                buf.push(format!("{} {}", a, b));
                return buf;
            }
        }
    }

    buf.push(format!("-1"));
    buf
}

#[test]
fn test_sample_1() {
    assert_eq!(run(106), vec!("4 2"));
}

#[test]
fn test_sample_2() {
    assert_eq!(run(1024), vec!("-1"));
}

#[test]
fn test_sample_3() {
    assert_eq!(run(10460353208), vec!("21 1"));
}
