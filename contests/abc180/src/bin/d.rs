fn main() {
    let (x, y, a, b) = {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        let mut ws = s.trim_end().split_whitespace();
        let x: u64 = ws.next().unwrap().parse().unwrap();
        let y: u64 = ws.next().unwrap().parse().unwrap();
        let a: u64 = ws.next().unwrap().parse().unwrap();
        let b: u64 = ws.next().unwrap().parse().unwrap();
        (x, y, a, b)
    };

    println!("{}", run(x, y, a, b));
}

fn run(begin_x: u64, y: u64, a: u64, b: u64) -> String {
    let mut x: u64 = begin_x;
    let mut exp: u64 = 0;
    let mut next_x = x * a;
    loop {
        if next_x > x + b {
            break;
        }
        if next_x >= y {
            break;
        }

        x = next_x;
        next_x = x * a;
        exp += 1;
    }
    if y >= x + 1 {
        exp = exp + (y - x - 1) / b;
    }

    format!("{}", exp)
}

#[test]
fn test_run_sample1() {
    assert_eq!(run(4, 20, 2, 10), "2");
}

#[test]
fn test_run_sample2() {
    assert_eq!(run(1, 1000000000000000000, 10, 1000000000), "1000000007");
}

#[test]
fn test_run_sample3() {
    assert_eq!(run(4, 1, 10, 10), "0");
}

#[test]
fn test_run_sample4() {
    assert_eq!(run(4, 4, 10, 10), "0");
}

#[test]
fn test_run_sample5() {
    assert_eq!(run(4, 10, 2, 20), "1");
}

#[test]
fn test_run_sample6() {
    assert_eq!(run(4, 16, 2, 40), "1");
}

#[test]
fn test_run_sample7() {
    assert_eq!(run(6, 30, 2, 10), "2");
}
